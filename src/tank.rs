use crate::{components::*, resources::*, *};
use bevy::math::f32;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::ExternalImpulse;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::events as pf_events;
use bevy_rts_pathfinding::flowfield::FlowField;
use bevy_rts_pathfinding::grid::Grid;
// use bevy_rts_pathfinding::grid_controller::GridController;
use events::SetUnitDestinationEv;

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tanks)
            .add_systems(Update, move_unit)
            .add_observer(set_unit_destination);
    }
}

fn spawn_tanks(mut cmds: Commands, assets: Res<AssetServer>) {
    let initial_pos_left = Vec3::new(-150.0, 0.0, 0.0);
    let initial_pos_right = Vec3::new(150.0, 0.0, 0.0);
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
            cmds.spawn(create_left_tank(row, col));
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
    mut q_unit: Query<Entity, With<pf_comps::Selected>>,
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

    for unit_entity in q_unit.iter_mut() {
        cmds.entity(unit_entity).insert(pf_comps::Destination);
    }

    cmds.trigger(pf_events::InitializeFlowFieldEv);
}

fn move_unit(
    mut q_unit: Query<(&mut Transform, &mut ExternalImpulse, &Speed), With<pf_comps::Destination>>,
    q_flowfield: Query<&FlowField>,
    time: Res<Time>,
) {
    let rotation_speed = 5.0;
    let rotation_threshold = 0.12;
    let delta_time = time.delta_secs();

    for flowfield in q_flowfield.iter() {
        for (mut unit_transform, mut ext_impulse, speed) in q_unit.iter_mut() {
            let cell = flowfield.get_cell_from_world_position(unit_transform.translation);
            let dir = cell.best_direction.vector();

            // Flatten direction on the XZ plane
            let raw_direction = Vec3::new(dir.x as f32, 0.0, dir.y as f32);

            if raw_direction.length_squared() == 0.0 {
                println!("No movement, skipping");
                continue;
            }

            let move_direction = raw_direction.normalize();

            // Current facing direction (flattened)
            let mut current_facing = (unit_transform.rotation * Vec3::Z).normalize();
            current_facing.y = 0.0;
            current_facing = current_facing.normalize();

            // Compute yaw angles using atan2(x, z) so that forward (0,0,1) = 0 radians
            let current_yaw = current_facing.x.atan2(current_facing.z);
            let target_yaw = move_direction.x.atan2(move_direction.z);

            // Compute the shortest angle difference
            let mut yaw_diff = target_yaw - current_yaw;
            if yaw_diff > std::f32::consts::PI {
                yaw_diff -= 2.0 * std::f32::consts::PI;
            } else if yaw_diff < -std::f32::consts::PI {
                yaw_diff += 2.0 * std::f32::consts::PI;
            }

            // Rotate or move depending on yaw difference
            if yaw_diff.abs() > rotation_threshold {
                let max_yaw_change = rotation_speed * delta_time;
                let clamped_yaw = yaw_diff.clamp(-max_yaw_change, max_yaw_change);
                let new_yaw = current_yaw + clamped_yaw;

                unit_transform.rotation = Quat::from_rotation_y(new_yaw);
            } else {
                let movement_impulse = move_direction * speed.0 * delta_time;
                ext_impulse.impulse += movement_impulse;
            }
        }
    }
}
