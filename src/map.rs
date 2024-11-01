use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rts_camera::Ground;
use pathfinding::prelude::astar;
use resources::MouseCoords;

use super::components::*;
use super::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .init_resource::<TargetCell>()
            .add_systems(Startup, spawn_map)
            .add_systems(Update, draw_grid);
    }
}

#[derive(Resource, Default)]
pub struct TargetCell {
    pub row: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Resource, Debug)]
pub struct Grid(pub Vec<Cell>);

#[derive(Debug)]
pub struct Cell {
    pub row: u32,
    pub column: u32,
    pub position: Vec2,
    pub walkable: bool,
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
                    walkable: true,
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
