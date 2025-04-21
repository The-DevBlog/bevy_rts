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
    // let desired_tile_size = 30.0;
    // let tile_factor = MAP_WIDTH / desired_tile_size;

    // // Build the mesh from the builder.
    // let mut mesh = Plane3d::default().mesh().size(MAP_WIDTH, MAP_DEPTH).build();

    // // Update the UVs so that the texture repeats.
    // if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute(Mesh::ATTRIBUTE_UV_0) {
    //     let new_uvs: Vec<[f32; 2]> = uvs
    //         .iter()
    //         .map(|uv| [uv[0] * tile_factor, uv[1] * tile_factor])
    //         .collect();

    //     if let Some(m) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
    //         *m = VertexAttributeValues::Float32x2(new_uvs);
    //     }
    // }

    // let plane_handle = meshes.add(mesh);

    // let material = StandardMaterial {
    //     base_color_texture: Some(my_assets.textures.grass_clr.clone()),
    //     normal_map_texture: Some(my_assets.textures.grass_normal.clone()),
    //     // metallic_roughness_texture: Some(my_assets.textures.grass_roughness.clone()), // super shiny
    //     perceptual_roughness: 1.0,
    //     occlusion_texture: Some(my_assets.textures.grass_occlusion.clone()),
    //     depth_bias: INFINITY,
    //     ..Default::default()
    // };

    // cmds.spawn((
    //     Mesh3d(plane_handle),
    //     MeshMaterial3d(materials.add(material)),
    //     Collider::cuboid(MAP_WIDTH / 2.0, 0.0, MAP_DEPTH / 2.0),
    //     Sensor,
    //     Ground,
    //     pf_comps::MapBase,
    //     Name::new("Map Base"),
    // ));

    // Ground
    // rgb(95, 123, 155)
    // rgb(111, 190, 111)
    let clr = Color::srgb(0.44, 0.75, 0.44); // green
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(MAP_WIDTH, MAP_DEPTH))),
        // MeshMaterial3d(extended_materials.add(ExtendedMaterial {
        //     base: StandardMaterial {
        //         base_color: clr,
        //         ..default()
        //     },
        //     extension: MyExtension {
        //         base_color: clr.into(),
        //         tint: TINT_CLR.into(),
        //         tint_strength: TINT_STRENGTH,
        //     },
        // })),
        MeshMaterial3d(materials.add(StandardMaterial {
            // base_color: Color::srgb(0.37, 0.48, 0.61), // blue
            base_color: Color::srgb(0.44, 0.75, 0.44), // green
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
    // Light
    let yaw = 150.0f32.to_radians() + std::f32::consts::PI; // 150° + 180° = 330° (in radians)
    let pitch = -40.0f32.to_radians();
    let roll = 0.0;

    cmds.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll)),
        Name::new("Light"),
    ));
}
