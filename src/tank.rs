use std::time::Duration;

use crate::{components::*, resources::*, *};

use bevy::math::f32;
use bevy::time::common_conditions::once_after_delay;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::ExternalImpulse;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::events as pf_events;
use bevy_rts_pathfinding::flowfield::FlowField;
use events::SetUnitDestinationEv;
use std::collections::HashMap;
pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_tanks)
        app.add_systems(
            Update,
            (
                // move_unit.run_if(any_with_component::<pf_comps::Destination>),
                move_units_boids_flowfield.run_if(any_with_component::<pf_comps::Destination>),
                spawn_tanks.run_if(once_after_delay(Duration::from_secs(1))),
                spawn_tank.run_if(once_after_delay(Duration::from_secs(1))),
            ),
        )
        .add_observer(set_unit_destination);
    }
}

pub fn spawn_tank(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((UnitBundle::new(
        "Tank".to_string(),
        TANK_SPEED * SPEED_QUANTIFIER,
        Vec3::new(4., 2., 6.),
        assets.load("tank_tan.glb#Scene0"),
        Transform::from_translation(Vec3::new(-100.0, 2.0, 0.0)),
    ),));
}

pub fn spawn_tanks(mut cmds: Commands, assets: Res<AssetServer>) {
    let initial_pos_left = Vec3::new(-150.0, 0.0, 0.0);
    let initial_pos_right = Vec3::new(500.0, 0.0, 0.0);
    let offset = Vec3::new(30.0, 0.0, 30.0);
    let grid_size = (TANK_COUNT as f32).sqrt().ceil() as usize;

    // Create tank on the left side facing right
    let create_left_tank = |row: usize, col: usize| {
        let pos = initial_pos_left + Vec3::new(offset.x * row as f32, 2.0, offset.z * col as f32);
        (UnitBundle::new(
            "Tank".to_string(),
            TANK_SPEED * SPEED_QUANTIFIER,
            Vec3::new(4., 2., 6.),
            assets.load("tank_tan.glb#Scene0"),
            Transform::from_translation(pos),
        ),)
    };

    // Create tank on the right side facing left
    let create_right_tank = |row: usize, col: usize| {
        let pos = initial_pos_right + Vec3::new(-offset.x * row as f32, 2.0, offset.z * col as f32);
        (UnitBundle::new(
            "Tank".to_string(),
            TANK_SPEED * SPEED_QUANTIFIER,
            Vec3::new(4., 2., 6.),
            assets.load("tank_tan.glb#Scene0"),
            Transform::from_translation(pos),
        ),)
    };

    // Spawn Left Group (facing right)
    let mut count = 0;
    for row in 0..grid_size {
        for col in 0..grid_size {
            if count >= TANK_COUNT {
                break;
            }
            // cmds.spawn(create_left_tank(row, col));
            count += 1;
        }
    }

    // Spawn Right Group (facing left)
    let mut count = 0;
    for row in 0..grid_size {
        for col in 0..grid_size {
            if count >= TANK_COUNT {
                break;
            }
            cmds.spawn(create_right_tank(row, col));
            count += 1;
        }
    }
}

pub fn set_unit_destination(
    _trigger: Trigger<SetUnitDestinationEv>,
    mouse_coords: ResMut<MouseCoords>,
    mut q_unit: Query<Entity, With<Selected>>,
    q_cam: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    q_rapier: Query<&RapierContext, With<DefaultRapierContext>>,
    mut cmds: Commands,
) {
    if !mouse_coords.in_bounds() {
        return;
    }

    let Ok(rapier_ctx) = q_rapier.get_single() else {
        return;
    };

    let (cam, cam_trans) = q_cam.single();
    let hit = utils::cast_ray(rapier_ctx, &cam, &cam_trans, mouse_coords.viewport);

    if let Some(_) = hit {
        return;
    }

    let mut units = Vec::new();
    for unit_entity in q_unit.iter_mut() {
        cmds.entity(unit_entity).insert(pf_comps::Destination);
        units.push(unit_entity);
    }

    cmds.trigger(pf_events::InitializeFlowFieldEv(units));
}

fn move_unit(
    mut q_unit: Query<(&mut Transform, &mut ExternalImpulse, &Speed), With<pf_comps::Destination>>,
    q_flowfield: Query<&FlowField>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for flowfield in q_flowfield.iter() {
        for &unit in &flowfield.units {
            if let Ok((mut unit_transform, mut ext_impulse, speed)) = q_unit.get_mut(unit) {
                let cell_below = flowfield.get_cell_from_world_position(unit_transform.translation);
                let raw_direction = Vec3::new(
                    cell_below.best_direction.vector().x as f32,
                    0.0,
                    cell_below.best_direction.vector().y as f32,
                )
                .normalize();

                if raw_direction.length_squared() > 0.000001 {
                    // Handle movement
                    let move_direction = raw_direction.normalize();
                    let yaw = f32::atan2(-move_direction.x, -move_direction.z);
                    unit_transform.rotation = Quat::from_rotation_y(yaw);

                    let movement_impulse = move_direction * speed.0 * delta_time;
                    ext_impulse.impulse += movement_impulse;
                }
            }
        }
    }
}

pub fn move_units_boids_flowfield(
    time: Res<Time>,
    mut q_boids: Query<
        (Entity, &mut Transform, &pf_comps::Boid, &Speed),
        With<pf_comps::Destination>,
    >,
    mut q_impulse: Query<&mut ExternalImpulse>,
    q_flowfields: Query<&FlowField>,
) {
    let dt = time.delta_secs();

    let mut boids_data = Vec::new();
    for (ent, position, boid, speed) in q_boids.iter() {
        boids_data.push((ent, position, boid, speed.0));
    }

    // We will also need a place to store the final "steering" for each entity:
    // Key is Entity, value is the final steering vector we compute.
    let mut steering_map: HashMap<Entity, Vec3> = HashMap::new();

    // === 2) For each flowfield, find relevant boids and compute boids forces + flowfield dir
    for flowfield in q_flowfields.iter() {
        // Filter down which boids are in this flowfield
        let relevant_boids: Vec<_> = boids_data
            .iter()
            .filter(|(ent, _, _, _)| flowfield.units.contains(ent))
            .collect();

        // For each boid, build neighbor list and compute boid vectors
        for (ent, pos, boid, _speed) in &relevant_boids {
            let my_pos = pos;
            // let boid = &boid_data.boid;

            // 2a) Gather neighbor positions
            let mut neighbor_positions = Vec::new();
            for (other_ent, other_pos, _, _) in &relevant_boids {
                if *other_ent == *ent {
                    continue;
                }
                let dist = my_pos.translation.distance(other_pos.translation);
                if dist < boid.neighbor_radius {
                    neighbor_positions.push(other_pos.translation);
                }
            }

            // 2b) Classical boids: separation, alignment, cohesion
            let mut separation = Vec3::ZERO;
            let mut alignment = Vec3::ZERO; // if you store velocity, do alignment properly
            let mut cohesion = Vec3::ZERO;

            if !neighbor_positions.is_empty() {
                // Separation
                for n_pos in &neighbor_positions {
                    let offset = my_pos.translation - *n_pos;
                    let dist = offset.length();
                    if dist > 0.0 {
                        separation += offset.normalize() / dist;
                    }
                }
                separation /= neighbor_positions.len() as f32;
                separation *= boid.separation_weight;

                // Cohesion
                let center =
                    neighbor_positions.iter().sum::<Vec3>() / neighbor_positions.len() as f32;
                let to_center = center - my_pos.translation;
                cohesion = to_center.normalize_or_zero() * boid.cohesion_weight;

                // Alignment – you’d need neighbor velocities to do it right
                alignment *= boid.alignment_weight;
            }

            // 2c) Flowfield direction
            let cell = flowfield.get_cell_from_world_position(my_pos.translation);
            let ff_dir_2d = cell.best_direction.vector();
            // Convert to 3D
            let ff_dir_3d = Vec3::new(ff_dir_2d.x as f32, 0.0, ff_dir_2d.y as f32);
            let flow_weight = 1.0; // if you want to tweak how strong flowfield is
            let flowfield_force = ff_dir_3d * flow_weight;

            // 2d) Sum up final steering
            let mut steering = separation + cohesion + alignment + flowfield_force;

            // Optionally clamp
            if steering.length() > boid.max_speed {
                steering = steering.normalize() * boid.max_speed;
            }

            // Store in the map so we can apply it later
            steering_map.insert(*ent, steering);
        }
    }

    // === 3) Now we do a second pass over the real transforms & impulses to apply changes
    for (ent, mut pos, _boid, speed) in q_boids.iter_mut() {
        // If we computed some steering for this entity, apply it
        if let Some(steering) = steering_map.get(&ent) {
            // Apply to impulse
            if let Ok(mut ext_impulse) = q_impulse.get_mut(ent) {
                let impulse_vec = *steering * speed.0 * dt;
                ext_impulse.impulse += impulse_vec;
            }

            // Apply to rotation
            if steering.length_squared() > 0.00001 {
                let yaw = f32::atan2(-steering.x, -steering.z);
                pos.rotation = Quat::from_rotation_y(yaw);
            }
        }
    }
}
