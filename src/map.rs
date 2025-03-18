use bevy::image::ImageAddressMode;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::{AddressMode, SamplerDescriptor};
use bevy::time::common_conditions::once_after_delay;
use bevy_rts_camera::Ground;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::grid::Grid;
use std::time::Duration;

use crate::resources::MyAssets;

use super::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_ground,
                spawn_light,
                // spawn_obstacle,
                spawn_grid,
            )
                .chain(),
        );
        // app.add_systems(
        //     Update,
        //     (
        // spawn_obstacle_2.run_if(once_after_delay(Duration::from_secs_f32(4.0))),
        // despawn_obstacles.run_if(once_after_delay(Duration::from_secs(6))),
        //     ),
        // );
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
    my_assets: Res<MyAssets>,
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
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(MAP_WIDTH, MAP_DEPTH))),
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
