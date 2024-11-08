use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Sensor};
use bevy_rts_camera::Ground;
use bevy_rts_pathfinding::components as pathfinding;

use super::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_map, spawn_grid, spawn_obstacle));
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
            mesh: meshes.add(Plane3d::default().mesh().size(MAP_WIDTH, MAP_HEIGHT)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Collider::cuboid(MAP_WIDTH / 2.0, 0.0, MAP_HEIGHT / 2.0),
        Sensor,
        Ground,
        pathfinding::MapBase,
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

fn spawn_grid(mut cmds: Commands) {
    let grid = (
        pathfinding::Grid::new(
            MAP_GRID_ROWS,
            MAP_GRID_COLUMNS,
            MAP_WIDTH,
            MAP_HEIGHT,
            MAP_CELL_WIDTH,
            MAP_CELL_HEIGHT,
        ),
        Name::new("Grid"),
    );

    let grid_colors = pathfinding::GridColors {
        grid: COLOR_GRID,
        path: COLOR_PATH,
        path_finding: COLOR_PATH_FINDING,
        occupied: COLOR_OCCUPIED_CELL,
    };

    cmds.spawn(grid);
    cmds.spawn(grid_colors);
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
