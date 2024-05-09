use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{Speed, TargetPos, Unit};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_unit);
    }
}

fn spawn_unit(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let unit = (
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(Color::DARK_GRAY),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        Name::new("Unit"),
        RigidBody::Dynamic,
        Speed(5.0),
        TargetPos(None),
        Unit,
    );

    let unit2 = (
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(2.0, 0.5, 0.0),
            material: materials.add(Color::DARK_GRAY),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        Name::new("Unit"),
        RigidBody::Dynamic,
        Speed(5.0),
        TargetPos(None),
        Unit,
    );

    cmds.spawn(unit);
    cmds.spawn(unit2);
}
