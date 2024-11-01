use bevy::prelude::*;
use bevy_mod_billboard::*;
use bevy_rapier3d::{plugin::RapierContext, prelude::*};
use events::SetUnitDestinationEv;
use map::{find_path, Grid, TargetCell};

use crate::{components::*, resources::*, utils, *};

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tanks)
            .add_systems(
                Update,
                (
                    move_unit::<Friendly>,
                    assign_path_system,
                    unit_movement_system,
                ),
            )
            .observe(set_unit_destination);
    }
}

fn spawn_tanks(mut cmds: Commands, assets: Res<AssetServer>, my_assets: Res<MyAssets>) {
    let initial_pos = Vec3::new(0.0, 0.0, 0.0);
    let offset = Vec3::new(20.0, 0.0, 20.0);
    let grid_size = (TANK_COUNT as f32).sqrt().ceil() as usize;

    let create_tank = |row: usize, col: usize| {
        let pos = initial_pos + Vec3::new(offset.x * row as f32, 2.0, offset.z * col as f32);
        (
            UnitBundle::new(
                "Tank".to_string(),
                TANK_SPEED * SPEED_QUANTIFIER,
                Vec3::new(4., 2., 6.),
                assets.load("tank.glb#Scene0"),
                pos,
            ),
            Selected(false),
            Friendly,
        )
    };

    let select_border = || {
        (
            BillboardTextureBundle {
                texture: BillboardTextureHandle(my_assets.select_border.clone()),
                billboard_depth: BillboardDepth(false),
                ..default()
            },
            UnitBorderBoxImg::new(15.0, 15.0),
            Name::new("Border Select"),
        )
    };

    let mut count = 0;
    for row in 0..grid_size {
        for col in 0..grid_size {
            if count >= TANK_COUNT {
                break;
            }
            cmds.spawn(create_tank(row, col)).with_children(|parent| {
                parent.spawn(select_border());
            });
            count += 1;
        }
    }
}

pub fn set_unit_destination(
    _trigger: Trigger<SetUnitDestinationEv>,
    mouse_coords: ResMut<MouseCoords>,
    mut friendly_q: Query<(&mut Destination, &Transform, &Selected), With<Friendly>>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
) {
    let (cam, cam_trans) = cam_q.single();
    let hit = utils::cast_ray(rapier_context, &cam, &cam_trans, mouse_coords.viewport);

    // return if selecting another object (select another unit for example)
    if let Some(_) = hit {
        return;
    }

    for (mut friendly_destination, trans, selected) in friendly_q.iter_mut() {
        if selected.0 {
            let mut destination = mouse_coords.world;
            destination.y += trans.scale.y / 2.0; // calculate for entity height
            friendly_destination.0 = Some(destination);
            // println!("Unit Moving to ({}, {})", destination.x, destination.y);
        }
    }
}

fn move_unit<T: Component>(
    mut unit_q: Query<
        (
            &mut CurrentAction,
            &mut Transform,
            &mut ExternalImpulse,
            &Speed,
            &mut Destination,
        ),
        With<T>,
    >,
    time: Res<Time>,
) {
    // for (mut action, mut trans, mut ext_impulse, speed, mut destination) in unit_q.iter_mut() {
    //     // only move the object if it has a destination
    //     if let Some(new_pos) = destination.0 {
    //         let distance = new_pos - trans.translation;
    //         let direction = Vec3::new(distance.x, 0.0, distance.z).normalize();
    //         rotate_towards(&mut trans, direction);

    //         if distance.length_squared() <= 5.0 {
    //             destination.0 = None;
    //             action.0 = Action::None;
    //         } else {
    //             action.0 = Action::Relocate;
    //             ext_impulse.impulse += direction * speed.0 * time.delta_seconds();
    //         }
    //     }
    // }
}

fn unit_movement_system(
    time: Res<Time>,
    mut units_query: Query<(&mut Transform, &mut DestinationPath), With<Friendly>>,
) {
    let delta_time = time.delta_seconds();

    for (mut transform, mut path_component) in units_query.iter_mut() {
        if !path_component.waypoints.is_empty() {
            // Get the next waypoint
            let target = path_component.waypoints[0];

            // Direction vector
            let direction = target - transform.translation;
            let distance = direction.length();

            if distance < 0.1 {
                // Reached the waypoint
                path_component.waypoints.remove(0);
                continue;
            }

            // Move towards the target
            let direction_normalized = direction.normalize();

            transform.translation += direction_normalized * TANK_SPEED * delta_time;

            // Optional: Rotate the unit to face the movement direction
            let target_rotation =
                Quat::from_rotation_y(-direction_normalized.x.atan2(direction_normalized.z));
            transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
        }
    }
}

fn assign_path_system(
    target_cell: Res<TargetCell>,
    grid: Res<Grid>,
    // selected_unit: Res<SelectedUnit>,
    mut units_query: Query<(&Transform, &mut DestinationPath, &mut Destination), With<Friendly>>,
) {
    // if let Some(selected_entity) = selected_unit.entity {
    if let (Some(goal_row), Some(goal_column)) = (target_cell.row, target_cell.column) {
        // if let Ok((transform, mut path_component, mut destination)) =
        //     units_query.get_mut(selected_entity)
        // {

        for (transform, mut path, mut destination) in units_query.iter_mut() {
            if destination.0.is_none() {
                continue;
            }

            // Get the unit's current cell
            let unit_pos = transform.translation;
            let grid_origin = -MAP_SIZE / 2.0;
            let adjusted_x = unit_pos.x - grid_origin;
            let adjusted_z = unit_pos.z - grid_origin;

            let start_column = (adjusted_x / MAP_CELL_SIZE).floor() as u32;
            let start_row = (adjusted_z / MAP_CELL_SIZE).floor() as u32;

            // Compute the path
            if let Some(path_indices) =
                find_path(&grid, (start_row, start_column), (goal_row, goal_column))
            {
                // Convert path indices to world positions
                let mut waypoints = Vec::new();
                for (path_row, path_column) in path_indices {
                    let index = (path_row * MAP_GRID_SIZE + path_column) as usize;
                    let cell = &grid.0[index];
                    let waypoint = Vec3::new(cell.position.x, unit_pos.y, cell.position.y);
                    waypoints.push(waypoint);
                }

                // Assign the path to the unit
                path.waypoints = waypoints.clone();

                // Set the unit's destination to the last waypoint
                destination.0 = Some(waypoints.last().cloned().unwrap());

                // println!("Assigned path to unit {:?}", selected_entity);
            }
        }
    }
    // }
}

fn rotate_towards(trans: &mut Transform, direction: Vec3) {
    let target_yaw = direction.x.atan2(direction.z);
    trans.rotation = Quat::from_rotation_y(target_yaw);
}
