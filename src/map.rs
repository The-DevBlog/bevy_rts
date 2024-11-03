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
            .add_systems(Update, set_grid_occupants)
            .add_systems(Update, draw_grid);
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
pub struct Grid(pub Vec<Cell>);

#[derive(Debug, Clone)]
pub struct Cell {
    pub row: u32,
    pub column: u32,
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

                let cell = Cell {
                    row,
                    column,
                    position,
                    occupied: true,
                };

                cells.push(cell);
            }
        }

        Grid(cells)
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
    let size = 12.0;
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(size, size, size)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            transform: Transform::from_translation(Vec3::new(100.0, 6.0, 100.0)),
            ..default()
        },
        Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
    ));
}

fn set_grid_occupants(
    mut grid: ResMut<Grid>,
    rapier_context: Res<RapierContext>,
    mut track: ResMut<SetGridOccupantsOnce>,
    time: Res<Time>,
    mut timer: ResMut<DelayedRunTimer>,
) {
    // Wait until the delay timer finishes, then run the system
    if !track.0 && timer.0.tick(time.delta()).finished() {
        let mut count = 0.0;

        // let buff = 0.35;
        let half_size = MAP_CELL_SIZE / 2.0;

        // Loop through each cell in the grid
        for cell in grid.0.iter_mut() {
            // count += 1.0;

            // Define the cell's bounding box as a Rapier cuboid (half extents of the cell)
            let cell_center = Vec3::new(cell.position.x, 0.0, cell.position.y);
            let cell_shape = Collider::cuboid(half_size, 1.0, half_size);

            if let Some(_) = rapier_context.intersection_with_shape(
                cell_center,
                Quat::IDENTITY,
                &cell_shape,
                QueryFilter::default().exclude_sensors(),
            ) {
                count += 1.0;
                cell.occupied = false;
            } else {
                cell.occupied = true;
            }
        }

        track.0 = true;
        println!("count: {}", count);
    }
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
