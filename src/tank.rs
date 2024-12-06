use crate::{components::*, resources::*, *};
// use bevy_mod_billboard::*;
use bevy_rapier3d::na::Rotation;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::ExternalImpulse;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::events as pf_events;
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

fn spawn_tanks(mut cmds: Commands, assets: Res<AssetServer>, my_assets: Res<MyAssets>) {
    let initial_pos_left = Vec3::new(-200.0, 0.0, 0.0); // Initial position for left group
    let initial_pos_right = Vec3::new(200.0, 0.0, 0.0); // Initial position for right group
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
                rotation: Quat::from_rotation_y(0.0), // Facing right (positive X direction)
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
                rotation: Quat::from_rotation_y(std::f32::consts::PI), // Facing left (negative X direction)
                ..default()
            },
        ),)
    };

    // Create select border for units
    let select_border = || {
        (
            // BillboardTextureBundle {
            //     texture: BillboardTextureHandle(my_assets.select_border.clone()),
            //     billboard_depth: BillboardDepth(false),
            //     ..default()
            // },
            UnitBorderBoxImg::new(15.0, 15.0),
            Name::new("Border Select"),
        )
    };

    // Spawn Left Group (facing right)
    let mut count = 0;
    for row in 0..grid_size {
        for col in 0..grid_size {
            if count >= TANK_COUNT {
                break;
            }
            cmds.spawn(create_left_tank(row, col))
                .with_children(|parent| {
                    parent.spawn(select_border());
                });
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
            cmds.spawn(create_right_tank(row, col))
                .with_children(|parent| {
                    parent.spawn(select_border());
                });
            count += 1;
        }
    }
}

pub fn set_unit_destination(
    _trigger: Trigger<SetUnitDestinationEv>,
    mouse_coords: ResMut<MouseCoords>,
    mut unit_q: Query<Entity, With<pf_comps::Selected>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    rapier_context: ReadDefaultRapierContext,
    mut cmds: Commands,
) {
    if !mouse_coords.in_bounds() {
        return;
    }

    let (cam, cam_trans) = cam_q.single();
    let hit = utils::cast_ray(rapier_context, &cam, &cam_trans, mouse_coords.viewport);

    if let Some(_) = hit {
        return;
    }

    for unit_entity in unit_q.iter_mut() {
        cmds.entity(unit_entity).insert(pf_comps::Destination);
    }

    cmds.trigger(pf_events::InitializeFlowFieldEv);
    // cmds.trigger(pf_events::SetTargetCellEv);
}

fn move_unit(// mut flowfield_q: Query<&mut pf_comps::FlowField>,
    // mut unit_q: Query<(&mut ExternalImpulse, &Transform, &Speed), With<pf_comps::Destination>>,
    // grid: Res<pf_res::Grid>,
    // time: Res<Time>,
    // mut cmds: Commands,
) {
    // if flowfield_q.is_empty() {
    //     return;
    // }

    // let delta_time = time.delta_seconds();
    // let rotation_speed = 5.0;
    // let movement_threshold = 15.0_f32.to_radians();

    // for mut flowfield in flowfield_q.iter_mut() {
    //     let mut units_to_remove = Vec::new();

    //     for &unit_entity in flowfield.entities.iter() {
    //         if let Ok((mut external_impulse, unit_transform, speed)) = unit_q.get_mut(unit_entity) {
    //             let (row, column) = pf_utils::get_cell(&grid, &unit_transform.translation);

    //             // Check if the unit has reached the destination cell
    //             if (row as usize, column as usize) == flowfield.destination {
    //                 units_to_remove.push(unit_entity);
    //                 continue;
    //             }

    //             let flow_vector = flowfield.cells[row as usize][column as usize].flow_vector;
    //             if flow_vector == Vec3::ZERO {
    //                 continue;
    //             }

    //             let desired_direction = flow_vector.normalize_or_zero();

    //             let forward = unit_transform.forward();
    //             let angle_difference = forward.angle_between(desired_direction);

    //             // Create a quaternion representing the rotation from `forward` to `desired_direction`
    //             let rotation_to_target = Quat::from_rotation_arc(*forward, desired_direction);

    //             // Convert the quaternion into an axis-angle representation
    //             let (rotation_axis, _) = rotation_to_target.to_axis_angle();

    //             let torque_impulse = rotation_axis * rotation_speed * delta_time * 20.0;
    //             external_impulse.torque_impulse += torque_impulse;

    //             if angle_difference > movement_threshold {
    //                 continue;
    //             }

    //             let movement_impulse = desired_direction * speed.0 * delta_time;
    //             external_impulse.impulse += movement_impulse;
    //         }
    //     }

    //     // Remove units that have reached the target or are invalid
    //     flowfield.entities.retain(|e| !units_to_remove.contains(e));
    // }

    // // TODO: Make this run every cell that is crossed, not every frame. This is expensive
    // cmds.trigger(pf_events::DetectCollidersEv);
}
