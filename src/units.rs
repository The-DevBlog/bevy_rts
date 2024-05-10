use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{Speed, TargetPos, Unit};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_unit);
    }
}

#[derive(Bundle)]
struct UnitBundle {
    pub collider: Collider,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub name: Name,
    pub rigid_body: RigidBody,
    pub speed: Speed,
    pub target_pos: TargetPos,
    pub unit: Unit,
}

impl UnitBundle {
    fn new(speed: f32, size: f32) -> Self {
        let size = size / 2.0;

        Self {
            collider: Collider::cuboid(size, size, size),
            damping: Damping {
                linear_damping: 10.0,
                ..default()
            },
            external_impulse: ExternalImpulse::default(),
            name: Name::new("Unit"),
            rigid_body: RigidBody::Dynamic,
            speed: Speed(speed),
            target_pos: TargetPos(None),
            unit: Unit,
        }
    }
}

fn spawn_unit(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut unit = |size: f32, speed: f32, translation: Vec3| -> (PbrBundle, UnitBundle) {
        (
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                transform: Transform {
                    translation: translation,
                    ..default()
                },
                material: materials.add(Color::DARK_GRAY),
                ..default()
            },
            UnitBundle::new(speed, size),
        )
    };

    cmds.spawn(unit(1.0, 60.0, Vec3::new(0.0, 0.5, 0.0)));
    cmds.spawn(unit(1.0, 60.0, Vec3::new(1.5, 0.5, 0.0)));
}
