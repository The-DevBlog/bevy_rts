use bevy::{color::palettes::css::LIGHT_GREEN, prelude::*};
use pathfinding::prelude::astar;

use crate::{components::*, map::*, resources::*, *};

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_line_to_destination,
                set_target_cell,
                set_destination_path,
            ),
        );
    }
}

fn draw_line_to_destination(
    unit_q: Query<(&Destination, &Transform), With<Friendly>>,
    mut gizmos: Gizmos,
) {
    for (destination, transform) in unit_q.iter() {
        if let Some(destination) = destination.0 {
            let unit_pos = transform.translation;
            gizmos.line(unit_pos, destination, COLOR_PATH_FINDING);
        }
    }
}

fn set_destination_path(
    grid: Res<Grid>,
    mut unit_q: Query<(&Transform, &Selected, &mut DestinationPath), With<Selected>>,
    target_cell: Res<TargetCell>,
    // input: Res<ButtonInput<MouseButton>>,
    mut gizmos: Gizmos,
) {
    for (transform, selected, mut destination_path) in unit_q.iter_mut() {
        if !selected.0 {
            continue;
        }

        if let (Some(goal_row), Some(goal_column)) = (target_cell.row, target_cell.column) {
            // Get the unit's current cell
            let unit_pos = transform.translation;
            let grid_origin = -MAP_SIZE / 2.0;
            let adjusted_x = unit_pos.x - grid_origin;
            let adjusted_z = unit_pos.z - grid_origin;

            let start_column = (adjusted_x / MAP_CELL_SIZE).floor() as u32;
            let start_row = (adjusted_z / MAP_CELL_SIZE).floor() as u32;

            // Compute the path
            if let Some(path) = find_path(&grid, (start_row, start_column), (goal_row, goal_column))
            {
                let mut waypoints: Vec<Cell> = Vec::new();

                // Highlight the path
                for &(row, column) in &path {
                    let index = (row * MAP_GRID_SIZE + column) as usize;
                    let cell = &grid.0[index];
                    waypoints.push(cell.clone());

                    // Draw a rectangle for each cell in the path
                    let position = Vec3::new(cell.position.x, 0.1, cell.position.y);
                    let rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
                    let size = Vec2::splat(MAP_CELL_SIZE);
                    let color = LIGHT_GREEN;

                    gizmos.rect(position, rotation, size, color);
                }

                // if input.just_pressed(MouseButton::Left) {
                //     destination_path.waypoints = waypoints;
                // }
            }
        }
    }
}

fn set_target_cell(mouse_coords: Res<MouseCoords>, mut target_cell: ResMut<TargetCell>) {
    // Adjust mouse coordinates to the grid's coordinate system
    let grid_origin = -MAP_SIZE / 2.0;
    let adjusted_x = mouse_coords.world.x - grid_origin; // Shift origin to (0, 0)
    let adjusted_z = mouse_coords.world.z - grid_origin;

    // Calculate the column and row indices
    let column = (adjusted_x / MAP_CELL_SIZE).floor() as u32;
    let row = (adjusted_z / MAP_CELL_SIZE).floor() as u32;

    // Check if indices are within the grid bounds
    if column < MAP_GRID_SIZE && row < MAP_GRID_SIZE {
        // println!("Mouse is over cell at row {}, column {}, position {:?}", cell.row, cell.column, cell.position);
        target_cell.row = Some(row);
        target_cell.column = Some(column);
    } else {
        target_cell.row = None;
        target_cell.column = None;
    }
}

pub fn find_path(grid: &Grid, start: (u32, u32), goal: (u32, u32)) -> Option<Vec<(u32, u32)>> {
    let result = astar(
        &start,
        |&(row, column)| successors(grid, row, column),
        |&(row, column)| heuristic(row, column, goal.0, goal.1),
        |&pos| pos == goal,
    );

    result.map(|(path, _cost)| path)
}

fn successors(grid: &Grid, row: u32, column: u32) -> Vec<((u32, u32), usize)> {
    let mut neighbors = Vec::new();
    let directions = [
        (-1, 0), // Up
        (1, 0),  // Down
        (0, -1), // Left
        (0, 1),  // Right
        // diagonal movement
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
        (1, -1),  // Down-Left
        (1, 1),   // Down-Right
    ];

    for (d_row, d_col) in directions.iter() {
        let new_row = row as i32 + d_row;
        let new_col = column as i32 + d_col;

        if new_row >= 0
            && new_row < MAP_GRID_SIZE as i32
            && new_col >= 0
            && new_col < MAP_GRID_SIZE as i32
        {
            let index = (new_row as u32 * MAP_GRID_SIZE + new_col as u32) as usize;
            let neighbor_cell = &grid.0[index];

            if neighbor_cell.walkable {
                neighbors.push(((new_row as u32, new_col as u32), 1)); // Cost is 1 per move
            }
        }
    }

    neighbors
}

fn heuristic(row: u32, column: u32, goal_row: u32, goal_column: u32) -> usize {
    let dx = (column as i32 - goal_column as i32).abs();
    let dy = (row as i32 - goal_row as i32).abs();
    (dx + dy) as usize // Manhattan distance
}
