use bevy_rts_camera::Ground;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::grid::Grid;

use super::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_light, spawn_grid).chain());
    }
}

fn spawn_grid(mut cmds: Commands) {
    let grid = Grid::new(IVec2::new(MAP_GRID_COLUMNS, MAP_GRID_ROWS), CELL_SIZE);
    cmds.insert_resource(grid);
}

fn spawn_ground(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(MAP_WIDTH, MAP_DEPTH))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: COLOR_GROUND,
            perceptual_roughness: 1.0,
            ..Default::default()
        })),
        Collider::cuboid(MAP_WIDTH / 2.0, 0.0, MAP_DEPTH / 2.0),
        Sensor,
        Ground,
        pf_comps::MapBase,
        Name::new("Map Base"),
    ));
}

fn spawn_light(mut cmds: Commands) {
    // Position the “sun” 10 units above the origin
    let light_height = 10.0;
    let light_pos = Vec3::new(0.0, light_height, 0.0);

    let mut transform = Transform::from_translation(light_pos).looking_at(Vec3::ZERO, Vec3::Y);

    let tilt: f32 = 15.0f32.to_radians();
    transform.rotate_x(tilt);

    cmds.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadow_depth_bias: 1.5,
            shadow_normal_bias: 1.0,
            shadows_enabled: true,
            ..default()
        },
        transform,
        Name::new("Sun Light"),
    ));
}
