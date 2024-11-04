use bevy::{color::palettes::css::LIGHT_GREEN, prelude::*};
use bevy_rapier3d::prelude::ExternalImpulse;
use pathfinding::prelude::astar;

use crate::{components::*, map::*, resources::*, *};

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_units_along_path,
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
    for (destination, unit_trans) in unit_q.iter() {
        if let Some(_) = destination.endpoint {
            let mut current = unit_trans.translation;

            for cell in destination.waypoints.iter() {
                let next = Vec3::new(cell.position.x, 0.1, cell.position.y);
                gizmos.line(current, next, COLOR_PATH_FINDING);
                current = next;
            }
        }
    }
}

fn move_units_along_path(
    time: Res<Time>,
    mut unit_q: Query<(
        &mut Transform,
        &mut Destination,
        &Speed,
        &mut ExternalImpulse,
    )>,
) {
    for (mut unit_trans, mut destination, speed, mut ext_impulse) in unit_q.iter_mut() {
        // println!("{:?}", destination.0);

        // Check if we've reached the end of the path
        if destination.waypoints.len() == 0 {
            destination.endpoint = None;
            *destination = Destination::default();
            continue;
        }

        // Get the current waypoint
        let cell = &destination.waypoints[0];
        let target_pos = Vec3::new(
            cell.position.x,
            unit_trans.translation.y, // Keep current y to avoid vertical movement
            cell.position.y,
        );

        // Calculate the direction and distance to the target position
        let direction = target_pos - unit_trans.translation;
        let distance_sq = direction.length_squared();

        let threshold = 5.0;
        if distance_sq < threshold {
            destination.waypoints.remove(0); // reach waypoint, remove it
        } else {
            // Move towards the waypoint
            let direction_normalized = Vec3::new(direction.x, 0.0, direction.z).normalize();
            tank::rotate_towards(&mut unit_trans, direction_normalized);
            ext_impulse.impulse += direction_normalized * speed.0 * time.delta_seconds();
        }
    }
}

fn set_destination_path(
    grid: Res<Grid>,
    mut unit_q: Query<(&Transform, &Selected, &mut Destination), With<Selected>>,
    target_cell: Res<TargetCell>,
    input: Res<ButtonInput<MouseButton>>,
) {
    for (transform, selected, mut destination) in unit_q.iter_mut() {
        if !selected.0 {
            continue;
        }

        if let (Some(goal_row), Some(goal_column)) = (target_cell.row, target_cell.column) {
            // Get the unit's current cell
            let (start_row, start_column) = utils::get_unit_cell_row_and_column(&transform);

            // Compute the path, ensuring only non-occupied cells are included
            if let Some(path) = find_path(&grid, (start_row, start_column), (goal_row, goal_column))
            {
                let mut waypoints: Vec<Cell> = Vec::new();

                // Highlight the path
                for &(row, column) in &path {
                    let cell = grid.cells[row as usize][column as usize];
                    waypoints.push(cell.clone());
                }

                // If a left mouse click is detected, assign the computed path
                if input.just_pressed(MouseButton::Left) {
                    destination.waypoints = waypoints;
                }
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
    astar(
        &start,
        |&(row, column)| successors(&grid, row, column),
        |&(row, column)| heuristic(row, column, goal.0, goal.1),
        |&pos| pos == goal,
    )
    .map(|(path, _cost)| path)
}

pub fn successors(grid: &Grid, row: u32, column: u32) -> Vec<((u32, u32), usize)> {
    let mut neighbors = Vec::new();
    let directions = [
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Up-Left (diagonal)
        (-1, 1),  // Up-Right (diagonal)
        (1, -1),  // Down-Left (diagonal)
        (1, 1),   // Down-Right (diagonal)
    ];

    for (d_row, d_col) in directions.iter() {
        let new_row = row as i32 + d_row;
        let new_col = column as i32 + d_col;

        if new_row >= 0
            && new_row < MAP_GRID_SIZE as i32
            && new_col >= 0
            && new_col < MAP_GRID_SIZE as i32
        {
            let neighbor_cell = grid.cells[new_row as usize][new_col as usize];

            // Only add the neighbor if it is not occupied
            if !neighbor_cell.occupied {
                neighbors.push(((new_row as u32, new_col as u32), 1)); // Cost is 1 per move
            }
        }
    }

    neighbors
}

pub fn heuristic(row: u32, column: u32, goal_row: u32, goal_column: u32) -> usize {
    let dx = (column as i32 - goal_column as i32).abs();
    let dy = (row as i32 - goal_row as i32).abs();
    (dx + dy) as usize // Manhattan distance
}
