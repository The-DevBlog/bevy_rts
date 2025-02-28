use bevy::math::f32;
use bevy::time::common_conditions::once_after_delay;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::ExternalImpulse;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::events as pf_events;
use bevy_rts_pathfinding::flowfield::FlowField;
use std::time::Duration;

use crate::{components::*, resources::*, *};
use events::SetUnitDestinationEv;

const TANK_SIZE: Vec3 = Vec3::new(4.8, 2.0, 7.8);
const BORDER_SIZE: Vec2 = Vec2::new(35.0, 35.0);

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tank);
        app.add_systems(
            Update,
            (
                // move_unit.run_if(any_with_component::<pf_comps::Destination>),
                set_is_moving,
                spawn_tanks.run_if(once_after_delay(Duration::from_secs(1))),
                // spawn_tank.run_if(once_after_delay(Duration::from_secs(1))),
                move_unit.run_if(any_with_component::<pf_comps::Destination>),
            )
                .chain(),
        )
        .add_observer(set_unit_destination);
    }
}

pub fn spawn_tank(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((UnitBundle::new(
        BORDER_SIZE,
        TANK_SPEED * SPEED_QUANTIFIER,
        "Tank".to_string(),
        assets.load("tank_tan.glb#Scene0"),
        TANK_SIZE,
        // Mesh3d(meshes.add(Cuboid::new(TANK_SIZE.x, TANK_SIZE.y, TANK_SIZE.z))), // TODO: remove
        // MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),              // TODO: remove
        Transform::from_translation(Vec3::new(-100.0, 2.0, 0.0)),
    ),));
}

pub fn spawn_tanks(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let initial_pos_left = Vec3::new(-150.0, 0.0, 0.0);
    let initial_pos_right = Vec3::new(500.0, 0.0, 0.0);
    let offset = Vec3::new(30.0, 0.0, 30.0);
    let grid_size = (TANK_COUNT as f32).sqrt().ceil() as usize;

    // let mesh = Mesh3d(meshes.add(Cuboid::new(TANK_SIZE.x, TANK_SIZE.y, TANK_SIZE.z)));
    // let material = MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3)));

    // Create tank on the left side facing right
    let create_left_tank = |row: usize, col: usize| {
        let pos = initial_pos_left + Vec3::new(offset.x * row as f32, 2.0, offset.z * col as f32);
        (UnitBundle::new(
            BORDER_SIZE,
            TANK_SPEED * SPEED_QUANTIFIER,
            "Tank".to_string(),
            assets.load("tank_tan.glb#Scene0"),
            TANK_SIZE,
            // mesh.clone(),     // TODO: remove
            // material.clone(), // TODO: remove
            Transform::from_translation(pos),
        ),)
    };

    // Create tank on the right side facing left
    let create_right_tank = |row: usize, col: usize| {
        let pos = initial_pos_right + Vec3::new(-offset.x * row as f32, 2.0, offset.z * col as f32);
        (UnitBundle::new(
            BORDER_SIZE,
            TANK_SPEED * SPEED_QUANTIFIER,
            "Tank".to_string(),
            assets.load("tank_tan.glb#Scene0"),
            TANK_SIZE,
            // mesh.clone(),     // TODO: remove
            // material.clone(), // TODO: remove
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

fn set_is_moving(mut q_is_moving: Query<(&mut IsMoving, &Velocity), With<Unit>>) {
    for (mut is_moving, velocity) in q_is_moving.iter_mut() {
        is_moving.0 = velocity.linvel.length_squared() > 0.0001;
    }
}

fn move_unit(
    q_ff: Query<&FlowField>,
    mut q_boids: Query<
        (Entity, &mut Transform, &pf_comps::Boid, &Speed),
        With<pf_comps::Destination>,
    >,
    mut q_impulse: Query<(&mut ExternalImpulse, &IsMoving), With<Unit>>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();
    for ff in q_ff.iter() {
        for (ent, mut pos, _boid, speed) in q_boids.iter_mut() {
            // Process the primary flow field steering
            if let Some(steering) = ff.flowfield_props.steering_map.get(&ent) {
                apply_steering(*steering, &mut pos, speed, delta_secs, ent, &mut q_impulse);
            }

            // Process the destination flow fields
            for dest_ff in ff.destination_flowfields.iter() {
                if let Some(steering) = dest_ff.flowfield_props.steering_map.get(&ent) {
                    apply_steering(*steering, &mut pos, speed, delta_secs, ent, &mut q_impulse);
                }
            }
        }
    }
}

fn apply_steering(
    steering: Vec3,
    pos: &mut Transform,
    speed: &Speed,
    delta_secs: f32,
    ent: Entity,
    q_impulse: &mut Query<(&mut ExternalImpulse, &IsMoving), With<Unit>>,
) {
    if steering.length_squared() > 0.00001 {
        let target_yaw = f32::atan2(-steering.x, -steering.z);
        let target_rotation = Quat::from_rotation_y(target_yaw);
        let rotation_speed = 5.0; // radians per second

        // apply rotation
        let angle_diff = pos.rotation.angle_between(target_rotation);
        if angle_diff > 0.0001 {
            let max_angle = rotation_speed * delta_secs;
            let t = (max_angle).min(angle_diff) / angle_diff;
            pos.rotation = pos.rotation.slerp(target_rotation, t);
        } else {
            pos.rotation = target_rotation;
        }

        // Only apply impulse if the rotation is nearly aligned with the target.
        if let Ok((mut ext_impulse, is_moving)) = q_impulse.get_mut(ent) {
            let rotation_threshold = 0.1; // radians
            if (is_moving.0 && angle_diff < 0.85) || (angle_diff < rotation_threshold) {
                // apply movement
                let impulse_vec = steering * speed.0 * delta_secs;
                ext_impulse.impulse += impulse_vec;
            }
        }
    }
}
