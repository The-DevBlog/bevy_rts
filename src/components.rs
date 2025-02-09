use bevy::{prelude::*, render::mesh};
use bevy_rapier3d::prelude::*;
use bevy_rts_pathfinding::components as pf_comps;

// TODO: Remove?
// #[derive(Component)]
// pub struct Unit;

#[derive(Component, Clone)]
pub struct UnitSelectBorder;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct SelectionBox;

#[derive(Bundle)]
pub struct UnitBundle {
    pub unit: pf_comps::Unit,
    pub size: pf_comps::UnitSize,
    pub collider: Collider,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub name: Name,
    pub rigid_body: RigidBody,
    pub speed: Speed,
    pub locked_axis: LockedAxes,
    pub transform: Transform,
    pub scene_root: SceneRoot, // TODO: uncomment
    // pub mesh: Mesh3d,
    // pub material: MeshMaterial3d<StandardMaterial>, // TODO: remove
    pub mass_properties: ColliderMassProperties, // TODO: remove
}

impl UnitBundle {
    pub fn new(
        name: String,
        speed: f32,
        size: Vec3,
        scene: Handle<Scene>,
        // mesh: Mesh3d, // TODO: remove
        // material: MeshMaterial3d<StandardMaterial>, // TODO: remove
        transform: Transform,
    ) -> Self {
        let scale_x = 1.2;
        let scale_z = 1.4;
        Self {
            mass_properties: ColliderMassProperties::MassProperties(MassProperties {
                principal_inertia: Vec3::ONE,
                mass: 1.0,
                ..default()
            }),
            unit: pf_comps::Unit,
            size: pf_comps::UnitSize(Vec2::new(size.x * scale_x, size.z * scale_z)), // TODO: uncomment
            collider: Collider::cuboid(size.x * scale_x, size.y, size.z * scale_z), // TODO: uncomment
            // size: pf_comps::UnitSize(Vec2::new(size.x, size.z)), // TODO: remove
            // collider: Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0), // TODO: remove
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
            scene_root: SceneRoot(scene), // TODO: uncomment
            // mesh, // TODO: remove
            // material, // TODO: remove
            transform,
        }
    }
}
