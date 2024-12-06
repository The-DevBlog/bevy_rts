use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::CURSOR_SIZE;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct UnitBorderBoxImg {
    pub width: f32,
    pub height: f32,
}

impl UnitBorderBoxImg {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct MyCursor {
    pub img: Handle<Image>,
    pub size: f32,
}

impl Default for MyCursor {
    fn default() -> Self {
        MyCursor {
            img: Handle::default(),
            size: CURSOR_SIZE,
        }
    }
}

#[derive(Component)]
pub struct SelectionBox;

#[derive(Bundle)]
pub struct UnitBundle {
    pub unit: Unit,
    pub collider: Collider,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub name: Name,
    pub rigid_body: RigidBody,
    pub speed: Speed,
    pub locked_axis: LockedAxes,
    pub transform: Transform,
    pub scene_root: SceneRoot,
    pub mass_properties: ColliderMassProperties,
}

impl UnitBundle {
    pub fn new(
        name: String,
        speed: f32,
        size: Vec3,
        scene: Handle<Scene>,
        transform: Transform,
    ) -> Self {
        Self {
            mass_properties: ColliderMassProperties::MassProperties(MassProperties {
                principal_inertia: Vec3::new(1.0, 1.0, 1.0),
                mass: 1.0,
                ..default()
            }),
            unit: Unit,
            collider: Collider::cuboid(size.x, size.y, size.z),
            damping: Damping {
                linear_damping: 10.0,
                angular_damping: 20.0,
                ..default()
            },
            external_impulse: ExternalImpulse::default(),
            name: Name::new(name),
            rigid_body: RigidBody::Dynamic,
            speed: Speed(speed),
            locked_axis: (LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Z
                | LockedAxes::TRANSLATION_LOCKED_Y),
            scene_root: SceneRoot(scene),
            transform,
        }
    }
}
