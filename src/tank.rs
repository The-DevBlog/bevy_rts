use bevy::prelude::*;
use bevy_mod_billboard::*;
// use bevy_rapier3d::plugin::RapierContext;
// use bevy_rts_pathfinding::components as pathfinding;
// use events::SetUnitDestinationEv;
use bevy_rts_pathfinding as pf;

use crate::{components::*, resources::*, utils, *};

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tanks);
        // .observe(set_unit_destination);
    }
}

// fn set_destination_path(
//     grid: Res<pf::Grid>,
//     // grid_q: Query<&Grid>,
//     mut unit_q: Query<(&Transform, &mut Destination), With<Selected>>,
//     target_cell: Res<TargetCell>,
//     input: Res<ButtonInput<MouseButton>>,
// ) {
//     // let grid = match grid_q.get_single() {
//     //     Ok(grid) => grid,
//     //     Err(_e) => return,
//     // };

//     for (unit_transform, mut destination) in unit_q.iter_mut() {
//         if let (Some(goal_row), Some(goal_column)) = (target_cell.row, target_cell.column) {
//             // Get the unit's current cell
//             let (start_row, start_column) = get_unit_cell_row_and_column(&grid, &unit_transform);

//             // Compute the path, ensuring only non-occupied cells are included
//             if let Some(path) = find_path(&grid, (start_row, start_column), (goal_row, goal_column))
//             {
//                 let mut waypoints: Vec<Cell> = Vec::new();

//                 // Highlight the path
//                 for &(row, column) in &path {
//                     let cell = grid.cells[row as usize][column as usize];
//                     waypoints.push(cell.clone());
//                 }

//                 // If a left mouse click is detected, assign the computed path
//                 if input.just_pressed(MouseButton::Left) {
//                     let destination_cell =
//                         grid.cells[goal_row as usize][goal_column as usize].position;
//                     destination.endpoint =
//                         Some(Vec3::new(destination_cell.x, 0.0, destination_cell.y));
//                     destination.waypoints = waypoints;
//                 }
//             }
//         }
//     }
// }

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
            // Unit,
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

// pub fn set_unit_destination(
//     _trigger: Trigger<SetUnitDestinationEv>,
//     mouse_coords: ResMut<MouseCoords>,
//     mut friendly_q: Query<(&mut pathfinding::Destination, &Transform), With<pathfinding::Selected>>,
//     cam_q: Query<(&Camera, &GlobalTransform)>,
//     rapier_context: Res<RapierContext>,
// ) {
//     let (cam, cam_trans) = cam_q.single();
//     let hit = utils::cast_ray(rapier_context, &cam, &cam_trans, mouse_coords.viewport);

//     // return if selecting another object (select another unit for example)
//     if let Some(_) = hit {
//         return;
//     }

//     for (mut friendly_destination, trans) in friendly_q.iter_mut() {
//         let mut destination = mouse_coords.world;
//         destination.y += trans.scale.y / 2.0; // calculate for entity height
//                                               // friendly_destination.endpoint = Some(destination);
//                                               // println!("Unit Moving to ({}, {})", destination.x, destination.y);
//     }
// }

// pub fn rotate_towards(trans: &mut Transform, direction: Vec3) {
//     let target_yaw = direction.x.atan2(direction.z);
//     trans.rotation = Quat::from_rotation_y(target_yaw);
// }
