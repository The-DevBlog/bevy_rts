use crate::CURSOR_SIZE;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// use bevy_rts_pathfinding::components as pathfinding;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct MapBase;

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
    // pub speed: pathfinding::Speed,
    // pub destination: pathfinding::Destination,
    pub locked_axis: LockedAxes,
    pub scene_bundle: SceneBundle,
}

impl UnitBundle {
    pub fn new(
        name: String,
        speed: f32,
        size: Vec3,
        scene: Handle<Scene>,
        translation: Vec3,
    ) -> Self {
        Self {
            unit: Unit,
            collider: Collider::cuboid(size.x, size.y, size.z),
            damping: Damping {
                linear_damping: 5.0,
                ..default()
            },
            external_impulse: ExternalImpulse::default(),
            name: Name::new(name),
            rigid_body: RigidBody::Dynamic,
            // speed: pathfinding::Speed(speed),
            // destination: pathfinding::Destination::default(),
            locked_axis: (LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Z
                | LockedAxes::ROTATION_LOCKED_Y
                | LockedAxes::TRANSLATION_LOCKED_Y),
            scene_bundle: SceneBundle {
                scene: scene,
                transform: Transform {
                    translation: translation,
                    ..default()
                },
                ..default()
            },
        }
    }
}
