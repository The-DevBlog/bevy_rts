use bevy::time::common_conditions::once_after_delay;
use bevy_rts_camera::Ground;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::grid::Grid;
use std::f32::INFINITY;
use std::time::Duration;

use super::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_map, spawn_obstacle, spawn_grid).chain());
        app.add_systems(
            Update,
            (
                spawn_obstacle_2.run_if(once_after_delay(Duration::from_secs_f32(4.0))),
                // despawn_obstacles.run_if(once_after_delay(Duration::from_secs(6))),
            ),
        );
        // TODO: Comment this back
        // .add_systems(
        //     Update,
        //     spawn_grid.run_if(once_after_delay(Duration::from_secs_f32(0.25))),
        // );
    }
}

fn spawn_grid(mut cmds: Commands) {
    let grid = Grid::new(IVec2::new(MAP_GRID_COLUMNS, MAP_GRID_ROWS), CELL_SIZE);
    cmds.insert_resource(grid);
}

fn spawn_map(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(MAP_WIDTH, MAP_DEPTH))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            depth_bias: INFINITY,
            ..Default::default()
        })),
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
        Transform::from_translation(Vec3::new(100.0, 6.0, 150.0)),
        Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
        RigidBody::Fixed,
        pf_comps::RtsObj,
        pf_comps::RtsObjSize(Vec3::new(size, size, size)),
    ));

    let obst = (
        Mesh3d(meshes.add(Cylinder::new(size, size / 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_translation(Vec3::new(-100.0, 6.0, 150.0)),
        Collider::cuboid(size, size / 2.0, size),
        RigidBody::Fixed,
        pf_comps::RtsObj,
        pf_comps::RtsObjSize(Vec3::new(size * 2.0, size, size * 2.0)),
    );

    let size = 125.0;
    let wall = (
        Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, size))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_translation(Vec3::new(-175.0, 2.5, 0.0)),
        Collider::cuboid(5.0 / 2.0, 5.0 / 2.0, size / 2.0),
        RigidBody::Fixed,
        pf_comps::RtsObj,
        pf_comps::RtsObjSize(Vec3::new(5.0, 5.0, size)),
    );

    cmds.spawn(obst);
    cmds.spawn(wall);
}

fn spawn_obstacle_2(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 12.0;
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(size, size, size))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_translation(Vec3::new(-100.0, 6.0, -150.0)),
        Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
        RigidBody::Fixed,
        pf_comps::RtsObj,
        pf_comps::RtsObjSize(Vec3::new(size, size, size)),
    ));

    let obst = (
        Mesh3d(meshes.add(Cylinder::new(size, size / 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_translation(Vec3::new(100.0, 6.0, -150.0)),
        Collider::cuboid(size, size / 2.0, size),
        RigidBody::Fixed,
        pf_comps::RtsObj,
        pf_comps::RtsObjSize(Vec3::new(size * 2.0, size * 2.0, size * 2.0)),
    );

    cmds.spawn(obst);
}
