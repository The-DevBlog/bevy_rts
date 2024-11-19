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
            .add_systems(Startup, (spawn_map, spawn_obstacle));
    }
}

fn spawn_grid(mut cmds: Commands) {
    // let grid = pf_res::Grid::new(MAP_GRID_ROWS, MAP_GRID_COLUMNS, CELL_SIZE);
    // cmds.insert_resource(grid);

    let grid_controller = (
        GridController {
            map_size: Vec2::new(MAP_WIDTH, MAP_DEPTH),
            grid_size: IVec2::new(MAP_GRID_ROWS, MAP_GRID_COLUMNS),
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
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(MAP_WIDTH, MAP_DEPTH)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Collider::cuboid(MAP_WIDTH / 2.0, 0.0, MAP_DEPTH / 2.0),
        Sensor,
        Ground,
        pf_comps::MapBase,
        Name::new("Map Base"),
    ));

    // Light
    cmds.spawn((
        DirectionalLightBundle {
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
        },
        Name::new("Light"),
    ));
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

    // let obst = (
    //     PbrBundle {
    //         mesh: meshes.add(Cylinder::new(size, size / 2.0)),
    //         material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
    //         transform: Transform::from_translation(Vec3::new(-100.0, 6.0, 100.0)),
    //         ..default()
    //     },
    //     Collider::cuboid(size, size / 2.0, size),
    // );

    // cmds.spawn(obst);
}
