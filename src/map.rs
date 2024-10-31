use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rts_camera::Ground;
use resources::MouseCoords;

use super::components::*;
use super::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .add_systems(Startup, spawn_map)
            .add_systems(Update, (draw_grid, detect_cell));
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

fn draw_grid(mut gizmos: Gizmos) {
    gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        UVec2::new(MAP_GRID_SIZE, MAP_GRID_SIZE),
        Vec2::new(MAP_CELL_SIZE, MAP_CELL_SIZE),
        ORANGE_RED,
    );
}

fn detect_cell(mut gizmos: Gizmos, grid: Res<Grid>, mouse_coords: Res<MouseCoords>) {
    // Adjust mouse coordinates to the grid's coordinate system
    let grid_origin = -MAP_SIZE / 2.0; // Grid starts at (-400.0, -400.0)
    let adjusted_x = mouse_coords.world.x - grid_origin; // Shift origin to (0, 0)
    let adjusted_z = mouse_coords.world.z - grid_origin;

    // Calculate the column and row indices
    let column = (adjusted_x / MAP_CELL_SIZE).floor() as u32;
    let row = (adjusted_z / MAP_CELL_SIZE).floor() as u32;

    // Check if indices are within the grid bounds
    if column < MAP_GRID_SIZE && row < MAP_GRID_SIZE {
        // Calculate the index in the grid's cell vector
        let index = (row * MAP_GRID_SIZE + column) as usize;
        let cell = &grid.0[index];

        // println!(
        //     "Mouse is over cell at row {}, column {}, position {:?}",
        //     cell.row, cell.column, cell.position
        // );

        // Draw a rectangle highlighting the cell
        let position = Vec3::new(cell.position.x, 0.1, cell.position.y);
        let rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2); // Align with XZ plane
        let size = Vec2::splat(MAP_CELL_SIZE);

        gizmos.rect(position, rotation, size, LIGHT_GREEN);
    } else {
        println!("Mouse is outside the grid");
    }
}

#[derive(Resource, Debug)]
pub struct Grid(pub Vec<Cell>);

#[derive(Debug)]
pub struct Cell {
    pub row: u32,
    pub column: u32,
    pub position: Vec2,
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

                let cell = Cell {
                    row,
                    column,
                    position,
                };

                cells.push(cell);
            }
        }

        Grid(cells)
    }
}
