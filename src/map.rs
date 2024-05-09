use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy_rts_camera::Ground;

use crate::MAP_SIZE;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

#[derive(Component)]
pub struct MapBase;

fn spawn_map(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(MAP_SIZE, MAP_SIZE)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, 0.0, MAP_SIZE / 2.0),
        Ground,
        MapBase,
        Name::new("Map Base"),
    ));

    // Some "terrain"
    let terrain_material = materials.add(Color::rgb(0.8, 0.7, 0.6));
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(15.0, 1.0, 5.0)),
            material: terrain_material.clone(),
            transform: Transform::from_xyz(15.0, 0.5, -5.0),
            ..default()
        },
        Collider::cuboid(7.5, 1.0, 2.5),
        Name::new("obstacle"),
        Ground,
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
