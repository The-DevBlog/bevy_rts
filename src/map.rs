use bevy::prelude::*;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::{Collider, QueryFilter, Sensor};
use bevy_rts_camera::Ground;

use super::components::*;
use super::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .init_resource::<TargetCell>()
            .init_resource::<SetGridOccupantsOnce>()
            .init_resource::<DelayedRunTimer>()
            .add_systems(Startup, (spawn_map, spawn_obstacle))
            .add_systems(
                Update,
                (draw_grid, set_grid_occupants, update_grid_occupants),
            );
    }
}

#[derive(Resource, Default)]
struct SetGridOccupantsOnce(pub bool);

#[derive(Resource)]
struct DelayedRunTimer(Timer);

impl Default for DelayedRunTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once)) // 0.5 seconds delay
    }
}

#[derive(Resource, Default)]
pub struct TargetCell {
    pub row: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Resource, Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    pub occupied_cells: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub position: Vec2,
    pub occupied: bool,
}

impl Default for Grid {
    fn default() -> Self {
        let mut cells = Vec::new();

        for row in 0..MAP_GRID_SIZE {
            for column in 0..MAP_GRID_SIZE {
                // Calculate the center position of each cell
                let position = Vec2::new(
                    -MAP_SIZE / 2.0 + column as f32 * MAP_CELL_SIZE + MAP_CELL_SIZE / 2.0,
                    -MAP_SIZE / 2.0 + row as f32 * MAP_CELL_SIZE + MAP_CELL_SIZE / 2.0,
                );

                cells.push(Cell {
                    position,
                    occupied: false,
                });
            }
        }

        Grid {
            cells,
            occupied_cells: Vec::default(),
        }
    }
}

fn spawn_map(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(MAP_SIZE, MAP_SIZE)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, 0.0, MAP_SIZE / 2.0),
        Sensor,
        Ground,
        MapBase,
        Name::new("Map Base"),
    ));

    // Light
    cmds.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            150.0f32.to_radians(),
            -40.0f32.to_radians(),
            0.0,
        )),
        ..default()
    });
}

fn spawn_obstacle(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let size = 12.0;
    // cmds.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Cuboid::new(size, size, size)),
    //         material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
    //         transform: Transform::from_translation(Vec3::new(100.0, 6.0, 100.0)),
    //         ..default()
    //     },
    //     Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
    // ));
}

// runs once at Update
fn set_grid_occupants(
    mut grid: ResMut<Grid>,
    rapier_context: Res<RapierContext>,
    mut track: ResMut<SetGridOccupantsOnce>,
    time: Res<Time>,
    mut timer: ResMut<DelayedRunTimer>,
) {
    // Wait until the delay timer finishes, then run the system
    if !track.0 && timer.0.tick(time.delta()).finished() {
        let half_size = MAP_CELL_SIZE / 2.0;

        let mut occupied_cells = Vec::new();

        // Loop through each cell in the grid
        for (idx, cell) in grid.cells.iter_mut().enumerate() {
            // Define the cell's bounding box as a Rapier cuboid (half extents of the cell)
            let cell_center = Vec3::new(cell.position.x, 0.0, cell.position.y);
            let cell_shape = Collider::cuboid(half_size, 1.0, half_size);

            if let Some(_) = rapier_context.intersection_with_shape(
                cell_center,
                Quat::IDENTITY,
                &cell_shape,
                QueryFilter::default().exclude_sensors(),
            ) {
                occupied_cells.push(idx);
                cell.occupied = true;
            }
        }

        grid.occupied_cells = occupied_cells;
        track.0 = true;
    }
}

fn update_grid_occupants(mut grid: ResMut<Grid>, rapier_context: Res<RapierContext>) {
    let half_size = MAP_CELL_SIZE / 2.0;

    // Create a new vector to hold indices of cells that are still occupied
    let mut still_occupied_cells = Vec::new();

    // Clone the occupied_cells list to iterate over to avoid borrowing issues
    let occupied_cells_snapshot = grid.occupied_cells.clone();

    // First pass: Check currently occupied cells and mark them as unoccupied if necessary
    for &idx in occupied_cells_snapshot.iter() {
        if let Some(cell) = grid.cells.get_mut(idx) {
            let cell_center = Vec3::new(cell.position.x, 0.0, cell.position.y);
            let cell_shape = Collider::cuboid(half_size, 1.0, half_size);

            // If cell is no longer occupied, mark it as unoccupied
            if rapier_context
                .intersection_with_shape(
                    cell_center,
                    Quat::IDENTITY,
                    &cell_shape,
                    QueryFilter::default().exclude_sensors(),
                )
                .is_none()
            {
                cell.occupied = false;
            } else {
                // If still occupied, add it to the new list
                still_occupied_cells.push(idx);
            }
        }
    }

    // Second pass: Check all cells to detect new occupied cells
    for (idx, cell) in grid.cells.iter_mut().enumerate() {
        // Skip cells that are already marked as occupied in the first pass
        if cell.occupied {
            continue;
        }

        let cell_center = Vec3::new(cell.position.x, 0.0, cell.position.y);
        let cell_shape = Collider::cuboid(half_size, 1.0, half_size);

        // If a unit is now within this cell, mark it as occupied
        if let Some(_) = rapier_context.intersection_with_shape(
            cell_center,
            Quat::IDENTITY,
            &cell_shape,
            QueryFilter::default().exclude_sensors(),
        ) {
            cell.occupied = true;
            still_occupied_cells.push(idx);
        }
    }

    // Update the grid's occupied cells list
    grid.occupied_cells = still_occupied_cells;
}

fn draw_grid(
    mut gizmos: Gizmos,
    mut unit_q: Query<(&Transform, &Selected), With<Selected>>,
    target_cell: Res<TargetCell>,
    grid: Res<Grid>,
) {
    // draw grid
    gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        UVec2::new(MAP_GRID_SIZE, MAP_GRID_SIZE),
        Vec2::new(MAP_CELL_SIZE, MAP_CELL_SIZE),
        COLOR_GRID,
    );

    // highlight unit paths
    for (unit_trans, selected) in unit_q.iter_mut() {
        if !selected.0 {
            continue;
        }
        if let (Some(goal_row), Some(goal_column)) = (target_cell.row, target_cell.column) {
            // Get the unit's current cell
            let (start_row, start_column) = utils::get_unit_cell_row_and_column(&unit_trans);

            // Compute the path, ensuring only non-occupied cells are included
            if let Some(path) =
                path_finding::find_path(&grid, (start_row, start_column), (goal_row, goal_column))
            {
                // Highlight the path
                for &(row, column) in &path {
                    let index = (row * MAP_GRID_SIZE + column) as usize;
                    let cell = &grid.cells[index];

                    // Draw a rectangle for each cell in the path
                    let position = Vec3::new(cell.position.x, 0.1, cell.position.y);
                    let rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
                    let size = Vec2::splat(MAP_CELL_SIZE);
                    let color = COLOR_PATH;

                    gizmos.rect(position, rotation, size, color);
                }
            }
        }
    }

    // highlight occupied cells
    for idx in &grid.occupied_cells {
        let cell = &grid.cells[*idx];
        let position = Vec3::new(cell.position.x, 0.1, cell.position.y);
        let rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
        let size = Vec2::splat(MAP_CELL_SIZE);
        gizmos.rect(position, rotation, size, COLOR_OCCUPIED_CELL);
    }
}
