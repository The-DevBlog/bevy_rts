use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rts_camera::Ground;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::flowfield::FlowField;
use bevy_rts_pathfinding::grid_controller::GridController;

use super::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_grid)
            .add_systems(Startup, (spawn_world, spawn_map, spawn_obstacle));
    }
}

fn spawn_world(mut cmds: Commands) {
    cmds.spawn(RapierContext::default());
}

fn spawn_grid(mut cmds: Commands) {
    let grid_controller = (
        GridController {
            grid_size: IVec2::new(MAP_GRID_COLUMNS, MAP_GRID_ROWS),
            cell_radius: CELL_SIZE / 2.,
            cur_flowfield: FlowField::default(),
        },
        Name::new("Grid Controller"),
    );

    cmds.spawn(grid_controller);
}

fn spawn_map(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(MAP_WIDTH, MAP_DEPTH))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Collider::cuboid(MAP_WIDTH / 2.0, 0.0, MAP_DEPTH / 2.0),
        Sensor,
        Ground,
        pf_comps::MapBase,
        Name::new("Map Base"),
    ));

    // Light
    cmds.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            150.0f32.to_radians(),
            -40.0f32.to_radians(),
            0.0,
        )),
        Name::new("Light"),
    ));
}

fn spawn_obstacle(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 12.0;
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(size, size, size))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_translation(Vec3::new(100.0, 6.0, 100.0)),
        Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
    ));

    let obst = (
        Mesh3d(meshes.add(Cylinder::new(size, size / 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_translation(Vec3::new(-100.0, 6.0, 100.0)),
        Collider::cuboid(size, size / 2.0, size),
    );

    cmds.spawn(obst);
}
