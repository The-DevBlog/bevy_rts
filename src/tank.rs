use crate::{components::*, resources::*, *};
use bevy::gizmos;
use bevy::math::f32;
use bevy_rapier3d::na::Rotation;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::ExternalImpulse;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::events as pf_events;
use bevy_rts_pathfinding::flowfield::FlowField;
use bevy_rts_pathfinding::grid_controller::GridController;
use bevy_rts_pathfinding::resources as pf_res;
use bevy_rts_pathfinding::utils as pf_utils;
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
            Transform {
                translation: pos,
                ..default()
            },
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
            Transform {
                translation: pos,
                ..default()
            },
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
    q_grid: Query<&GridController>,
    time: Res<Time>,
) {
    let Ok(grid) = q_grid.get_single() else {
        return;
    };

    if grid.cur_flowfield == FlowField::default() {
        return;
    }

    let delta_time = time.delta_secs();

    for (mut unit_transform, mut ext_impulse, speed) in q_unit.iter_mut() {
        let cell_below = grid
            .cur_flowfield
            .get_cell_from_world_position(unit_transform.translation);

        let raw_direction = Vec3::new(
            cell_below.best_direction.vector().x as f32,
            0.0,
            cell_below.best_direction.vector().y as f32,
        )
        .normalize();

        // Only update rotation and movement if there is a meaningful direction.
        if raw_direction.length_squared() > 0.000001 {
            let move_direction = raw_direction.normalize();

            // Compute yaw assuming forward is along +Z axis.
            let yaw = f32::atan2(-move_direction.x, -move_direction.z);

            // Only update rotation if direction is non-zero
            unit_transform.rotation = Quat::from_rotation_y(yaw);

            // Apply movement
            let movement_impulse = move_direction * speed.0 * delta_time;
            ext_impulse.impulse += movement_impulse;
        }
    }
}

// fn move_unit(
//     mut q_unit: Query<(&mut Transform, &mut ExternalImpulse, &Speed), With<pf_comps::Destination>>,
//     q_grid: Query<&GridController>,
//     time: Res<Time>,
// ) {
//     let Ok(grid) = q_grid.get_single() else {
//         return;
//     };

//     if grid.cur_flowfield == FlowField::default() {
//         return;
//     }

//     let delta_time = time.delta_secs();

//     for (mut unit_transform, mut ext_impulse, speed) in q_unit.iter_mut() {
//         let cell_below = grid
//             .cur_flowfield
//             .get_cell_from_world_position(unit_transform.translation);

//         let move_direction = Vec3::new(
//             cell_below.best_direction.vector().x as f32,
//             unit_transform.translation.y,
//             cell_below.best_direction.vector().y as f32,
//         )
//         .normalize();

//         // If the direction is valid (not zero), face towards it.
//         // if move_direction.length_squared() > 0.0 {
//         let yaw = f32::atan2(-move_direction.x, -move_direction.z);

//         // Apply the rotation to face the direction of movement
//         unit_transform.rotation = Quat::from_rotation_y(yaw);

//         // Move the unit along that direction
//         let movement_impulse = move_direction * speed.0 * delta_time;
//         ext_impulse.impulse += movement_impulse;
//         // }
//     }
// }

// fn move_unit(
//     mut q_unit: Query<(&Transform, &mut ExternalImpulse, &Speed), With<pf_comps::Destination>>,
//     q_grid: Query<&GridController>,
//     time: Res<Time>,
// ) {
//     let Ok(grid) = q_grid.get_single() else {
//         return;
//     };

//     if grid.cur_flowfield == FlowField::default() {
//         return;
//     }

//     let delta_time = time.delta_secs();

//     for (unit_transform, mut ext_impulse, speed) in q_unit.iter_mut() {
//         let cell_below = grid
//             .cur_flowfield
//             .get_cell_from_world_position(unit_transform.translation);

//         let move_direction = Vec3::new(
//             cell_below.best_direction.vector().x as f32,
//             unit_transform.translation.y,
//             cell_below.best_direction.vector().y as f32,
//         )
//         .normalize();

//         // Move Unit
//         let movement_impulse = move_direction * speed.0 * delta_time;
//         ext_impulse.impulse += movement_impulse;
//     }
// }

// fn move_unit_2(
//     mut q_unit: Query<(&Transform, &mut ExternalImpulse, &Speed), With<pf_comps::Destination>>,
//     q_grid: Query<&GridController>,
//     time: Res<Time>,
//     mut gizmos: Gizmos,
// ) {
//     let Ok(grid) = q_grid.get_single() else {
//         return;
//     };

//     if grid.cur_flowfield == FlowField::default() {
//         return;
//     }

//     let delta_time = time.delta_secs();
//     let rotation_speed = 5.0;
//     let movement_threshold = 15.0_f32.to_radians();

//     for (unit_transform, mut ext_impulse, speed) in q_unit.iter_mut() {
//         let cell_below = grid
//             .cur_flowfield
//             .get_cell_from_world_position(unit_transform.translation);

//         let move_direction = Vec3::new(
//             cell_below.best_direction.vector().x as f32,
//             unit_transform.translation.y,
//             cell_below.best_direction.vector().y as f32,
//         )
//         .normalize();

//         let forward = unit_transform.forward();

//         let rotation_to_target = Quat::from_rotation_arc(*forward, move_direction);

//         // Project onto the XZ plane and normalize
//         let forward_2d = Vec2::new(forward.x, forward.z).normalize();
//         let move_2d = Vec2::new(move_direction.x, move_direction.z).normalize();

//         // Compute the angle between them (still returns 0 to π)
//         let unsigned_angle = forward_2d.angle_to(move_2d);

//         // To get the sign, use the perpendicular dot product (perp_dot)
//         let perp_dot = forward_2d.perp_dot(move_2d);
//         let sign = perp_dot.signum();

//         let (rotation_axis, rotation_angle) = rotation_to_target.to_axis_angle();
//         let torque_impulse = rotation_axis * rotation_speed * delta_time * 20.0;
//         // let torque_impulse = rotation_axis * rotation_speed * delta_time * 20.0;
//         ext_impulse.torque_impulse += torque_impulse;

//         // Only move unit if it is facing the move direction
//         let angle_difference = unsigned_angle * sign;
//         if angle_difference > movement_threshold {
//             continue;
//         }

//         // Move Unit
//         let movement_impulse = move_direction * speed.0 * delta_time;
//         ext_impulse.impulse += movement_impulse;
//     }
// }
