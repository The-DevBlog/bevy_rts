use std::time::Duration;

use super::*;

use bevy::time::common_conditions::once_after_delay;
use bevy_rts_camera::Ground;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::grid::Grid;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyTimer>()
            .add_systems(Startup, (spawn_map))
            .add_systems(
                Update,
                (
                    spawn_grid,
                    spawn_obstacle.run_if(once_after_delay(Duration::from_secs(1))),
                ),
            );
    }
}

#[derive(Resource)]
struct MyTimer(Timer);

impl Default for MyTimer {
    fn default() -> Self {
        MyTimer(Timer::from_seconds(0.25, TimerMode::Once))
    }
}

fn spawn_grid(
    mut cmds: Commands,
    mut timer: ResMut<MyTimer>,
    time: Res<Time>,
    q_rapier: Query<&RapierContext>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    let Ok(rapier_ctx) = q_rapier.get_single() else {
        println!("No rapier ctx found");
        return;
    };

    let collision_checker = |world_pos| {
        let cell_radius = CELL_SIZE / 2.0;
        rapier_ctx
            .intersection_with_shape(
                world_pos,
                Quat::IDENTITY,
                &Collider::cuboid(cell_radius, cell_radius, cell_radius),
                QueryFilter::default().exclude_sensors(),
            )
            .is_some()
    };

    let grid = Grid::new(
        IVec2::new(MAP_GRID_COLUMNS, MAP_GRID_ROWS),
        CELL_SIZE,
        collision_checker,
    );

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
    // cmds.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(size, size, size))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    //     Transform::from_translation(Vec3::new(100.0, 6.0, 150.0)),
    //     Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
    // ));

    let obst = (
        Mesh3d(meshes.add(Cylinder::new(size, size / 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_translation(Vec3::new(-100.0, 6.0, 150.0)),
        Collider::cuboid(size, size / 2.0, size),
    );

    // cmds.spawn(obst);
}
