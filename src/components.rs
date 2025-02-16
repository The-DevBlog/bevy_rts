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
    pub collider: Collider,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub locked_axis: LockedAxes,
    pub mass_properties: ColliderMassProperties, // TODO: remove
    pub name: Name,
    pub rigid_body: RigidBody,
    pub scene_root: SceneRoot, // TODO: uncomment
    pub size: pf_comps::UnitSize,
    pub speed: Speed,
    pub transform: Transform,
    pub unit: pf_comps::Unit,
    // pub mesh: Mesh3d,
    // pub material: MeshMaterial3d<StandardMaterial>, // TODO: remove
}

impl UnitBundle {
    pub fn new(
        speed: f32,
        // mesh: Mesh3d, // TODO: remove
        // material: MeshMaterial3d<StandardMaterial>, // TODO: remove
        name: String,
        scene: Handle<Scene>,
        size: Vec3,
        transform: Transform,
    ) -> Self {
        let scale_x = 1.2;
        let scale_z = 1.4;
        Self {
            collider: Collider::cuboid(size.x * scale_x, size.y, size.z * scale_z), // TODO: uncomment
            // collider: Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0), // TODO: remove
            damping: Damping {
                linear_damping: 10.0,
                angular_damping: 20.0,
                ..default()
            },
            external_impulse: ExternalImpulse::default(),
            name: Name::new(name),
            locked_axis: (LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Z
                | LockedAxes::TRANSLATION_LOCKED_Y),
            mass_properties: ColliderMassProperties::MassProperties(MassProperties {
                principal_inertia: Vec3::ONE,
                mass: 1.0,
                ..default()
            }),
            rigid_body: RigidBody::Dynamic,
            scene_root: SceneRoot(scene), // TODO: uncomment
            size: pf_comps::UnitSize(Vec2::new(size.x * scale_x, size.z * scale_z)), // TODO: uncomment
            // size: pf_comps::UnitSize(Vec2::new(size.x, size.z)), // TODO: remove
            speed: Speed(speed),
            transform,
            unit: pf_comps::Unit,
            // mesh, // TODO: remove
            // material, // TODO: remove
        }
    }
}
