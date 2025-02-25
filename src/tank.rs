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

// fn move_unit(
//     mut q_unit: Query<(&mut Transform, &mut ExternalImpulse, &Speed), With<pf_comps::Destination>>,
//     q_ff: Query<&FlowField>,
//     time: Res<Time>,
// ) {
//     let delta_time = time.delta_secs();

//     for ff in q_ff.iter() {
//         for &unit in &ff.flowfield_props.units {
//             if let Ok((mut unit_transform, mut ext_impulse, speed)) = q_unit.get_mut(unit) {
//                 let cell_below = ff.get_cell_from_world_position(unit_transform.translation);
//                 let raw_direction = Vec3::new(
//                     cell_below.best_direction.vector().x as f32,
//                     0.0,
//                     cell_below.best_direction.vector().y as f32,
//                 )
//                 .normalize();

//                 if raw_direction.length_squared() > 0.000001 {
//                     // Handle movement
//                     let move_direction = raw_direction.normalize();
//                     let yaw = f32::atan2(-move_direction.x, -move_direction.z);
//                     unit_transform.rotation = Quat::from_rotation_y(yaw);

//                     let movement_impulse = move_direction * speed.0 * delta_time;
//                     ext_impulse.impulse += movement_impulse;
//                 }
//             }
//         }

//         for dest_ff in ff.destination_flowfields.iter() {
//             for &unit in dest_ff.flowfield_props.units.iter() {
//                 if let Ok((mut unit_transform, mut ext_impulse, speed)) = q_unit.get_mut(unit) {
//                     let cell_below =
//                         dest_ff.get_cell_from_world_position(unit_transform.translation);

//                     let raw_direction = Vec3::new(
//                         cell_below.best_direction.vector().x as f32,
//                         0.0,
//                         cell_below.best_direction.vector().y as f32,
//                     )
//                     .normalize();

//                     if raw_direction.length_squared() > 0.000001 {
//                         // Handle movement
//                         let move_direction = raw_direction.normalize();
//                         let yaw = f32::atan2(-move_direction.x, -move_direction.z);
//                         unit_transform.rotation = Quat::from_rotation_y(yaw);

//                         let movement_impulse = move_direction * speed.0 * delta_time;
//                         ext_impulse.impulse += movement_impulse;
//                     }
//                 }
//             }
//         }
//         // }
//     }
// }

// // TODO: Move this logic to crate side?
// fn move_unit(
//     q_ff: Query<&FlowField>,
//     mut q_boids: Query<
//         (Entity, &mut Transform, &pf_comps::Boid, &Speed),
//         With<pf_comps::Destination>,
//     >,
//     mut q_impulse: Query<&mut ExternalImpulse>,
//     time: Res<Time>,
// ) {
//     let delta_secs = time.delta_secs();
//     for ff in q_ff.iter() {
//         for (ent, mut pos, _boid, speed) in q_boids.iter_mut() {
//             if let Some(steering) = ff.flowfield_props.steering_map.get(&ent) {
//                 // This code works, but it just snaps to the target rotation instantly.
//                 {
//                     // Apply to impulse
//                     // if let Ok(mut ext_impulse) = q_impulse.get_mut(ent) {
//                     //     let impulse_vec = *steering * speed.0 * delta_secs;
//                     //     ext_impulse.impulse += impulse_vec;
//                     // }

//                     // Apply to rotation
//                     // if steering.length_squared() > 0.00001 {
//                     //     let yaw = f32::atan2(-steering.x, -steering.z);
//                     //     pos.rotation = Quat::from_rotation_y(yaw);
//                     // }
//                 }

//                 // this code works, but the boid separation is not working. Maybe need to adjust boid fields?
//                 {
//                     // Apply to rotation
//                     if steering.length_squared() > 0.00001 {
//                         // Compute the desired yaw from your steering vector.
//                         let target_yaw = f32::atan2(-steering.x, -steering.z);
//                         // Create a quaternion representing the target rotation.
//                         let target_rotation = Quat::from_rotation_y(target_yaw);
//                         let rotation_speed = 5.0; // in radians per second

//                         // Compute the angle between the current and target rotations.
//                         let angle_diff = pos.rotation.angle_between(target_rotation);
//                         if angle_diff > 0.0001 {
//                             // Determine the maximum angle you can rotate this frame.
//                             let max_angle = rotation_speed * delta_secs;
//                             // Figure out how far along the interpolation we can move.
//                             // If max_angle is greater than angle_diff, we can snap directly.
//                             let t = (max_angle).min(angle_diff) / angle_diff;
//                             // Slerp towards the target rotation.
//                             pos.rotation = pos.rotation.slerp(target_rotation, t);
//                         } else {
//                             // If we're extremely close, just set it directly.
//                             pos.rotation = target_rotation;
//                         }

//                         // Only apply impulse if the rotation is nearly aligned with the target.
//                         let rotation_threshold = 0.1; // in radians; adjust as needed
//                         if angle_diff < rotation_threshold {
//                             if let Ok(mut ext_impulse) = q_impulse.get_mut(ent) {
//                                 let impulse_vec = *steering * speed.0 * delta_secs;
//                                 ext_impulse.impulse += impulse_vec;
//                             }
//                         }
//                     }
//                 }
//             }

//             for dest_ff in ff.destination_flowfields.iter() {
//                 if let Some(steering) = dest_ff.flowfield_props.steering_map.get(&ent) {
//                     // println!("Steering: {:?}", steering);

//                     // This code works, but it just snaps to the target rotation instantly.
//                     {
//                         // Apply to impulse
//                         // if let Ok(mut ext_impulse) = q_impulse.get_mut(ent) {
//                         //     let impulse_vec = *steering * speed.0 * delta_secs;
//                         //     ext_impulse.impulse += impulse_vec;
//                         // }

//                         // Apply to rotation
//                         // if steering.length_squared() > 0.00001 {
//                         //     let yaw = f32::atan2(-steering.x, -steering.z);
//                         //     pos.rotation = Quat::from_rotation_y(yaw);
//                         // }
//                     }

//                     // this code works, but the boid separation is not working. Maybe need to adjust boid fields?
//                     {
//                         // Apply to rotation
//                         if steering.length_squared() > 0.00001 {
//                             // Compute the desired yaw from your steering vector.
//                             let target_yaw = f32::atan2(-steering.x, -steering.z);
//                             // Create a quaternion representing the target rotation.
//                             let target_rotation = Quat::from_rotation_y(target_yaw);
//                             let rotation_speed = 5.0; // in radians per second

//                             // Compute the angle between the current and target rotations.
//                             let angle_diff = pos.rotation.angle_between(target_rotation);
//                             if angle_diff > 0.0001 {
//                                 // Determine the maximum angle you can rotate this frame.
//                                 let max_angle = rotation_speed * delta_secs;
//                                 // Figure out how far along the interpolation we can move.
//                                 // If max_angle is greater than angle_diff, we can snap directly.
//                                 let t = (max_angle).min(angle_diff) / angle_diff;
//                                 // Slerp towards the target rotation.
//                                 pos.rotation = pos.rotation.slerp(target_rotation, t);
//                             } else {
//                                 // If we're extremely close, just set it directly.
//                                 pos.rotation = target_rotation;
//                             }

//                             // Only apply impulse if the rotation is nearly aligned with the target.
//                             let rotation_threshold = 0.1; // in radians; adjust as needed
//                             if angle_diff < rotation_threshold {
//                                 if let Ok(mut ext_impulse) = q_impulse.get_mut(ent) {
//                                     let impulse_vec = *steering * speed.0 * delta_secs;
//                                     ext_impulse.impulse += impulse_vec;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

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
        // Compute the desired yaw from your steering vector.
        let target_yaw = f32::atan2(-steering.x, -steering.z);
        // Create a quaternion representing the target rotation.
        let target_rotation = Quat::from_rotation_y(target_yaw);
        let rotation_speed = 5.0; // radians per second

        // Compute the angle between the current and target rotations.
        let angle_diff = pos.rotation.angle_between(target_rotation);
        if angle_diff > 0.0001 {
            // Determine the maximum angle you can rotate this frame.
            let max_angle = rotation_speed * delta_secs;
            // Figure out how far along the interpolation we can move.
            let t = (max_angle).min(angle_diff) / angle_diff;
            // Slerp towards the target rotation.
            pos.rotation = pos.rotation.slerp(target_rotation, t);
        } else {
            pos.rotation = target_rotation;
        }

        // Only apply impulse if the rotation is nearly aligned with the target.
        if let Ok((mut ext_impulse, is_moving)) = q_impulse.get_mut(ent) {
            let rotation_threshold = 0.1; // radians; adjust as needed
            if (is_moving.0 && angle_diff < 0.85) || (angle_diff < rotation_threshold) {
                let impulse_vec = steering * speed.0 * delta_secs;
                ext_impulse.impulse += impulse_vec;
            }
        }
    }
}
