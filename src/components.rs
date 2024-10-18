use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Selected(pub bool);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct BorderSelect {
    pub width: f32,
    pub height: f32,
}

impl BorderSelect {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Component)]
pub struct Target(pub Option<Entity>);

#[derive(Component)]
pub struct Friendly;

#[derive(Component)]
pub struct Destination(pub Option<Vec3>);

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct MapBase;

#[derive(Component, Debug)]
pub struct CurrentAction(pub Action);

#[derive(Debug, PartialEq)]
pub enum Action {
    Attack,
    Relocate,
    None,
}

#[derive(Bundle)]
pub struct UnitBundle {
    pub collider: Collider,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub name: Name,
    pub rigid_body: RigidBody,
    pub speed: Speed,
    pub destination: Destination,
    pub unit: Unit,
    pub locked_axis: LockedAxes,
    pub scene_bundle: SceneBundle,
    pub current_action: CurrentAction,
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
            collider: Collider::cuboid(size.x, size.y, size.z),
            damping: Damping {
                linear_damping: 5.0,
                ..default()
            },
            external_impulse: ExternalImpulse::default(),
            name: Name::new(name),
            rigid_body: RigidBody::Dynamic,
            speed: Speed(speed),
            destination: Destination(None),
            unit: Unit,
            current_action: CurrentAction(Action::None),
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
