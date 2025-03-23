use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rts_pathfinding::components as pf_comps;
use strum_macros::EnumIter;

use crate::resources::MyAssets;

#[derive(Component, Clone)]
pub struct UnitSelectBorder(pub Entity);

#[derive(Component)]
pub struct BorderSize(pub Vec2);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct SelectionBox;

#[derive(Component, Default)]
pub struct IsMoving(pub bool);

#[derive(Component, EnumIter)]
#[require(pf_comps::RtsObj, IsMoving, Velocity)]
pub enum Unit {
    TankGen1,
    TankGen2,
}

impl Unit {
    pub fn cost(&self) -> f32 {
        match self {
            Unit::TankGen1 => 500.0,
            Unit::TankGen2 => 800.0,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Unit::TankGen1 => "Tank Gen I",
            Unit::TankGen2 => "Tank Gen II",
        }
    }

    pub fn img(&self, my_assets: &Res<MyAssets>) -> Handle<Image> {
        match self {
            Unit::TankGen1 => my_assets.imgs.unit_tank_gen1.clone(),
            Unit::TankGen2 => my_assets.imgs.unit_tank_gen2.clone(),
        }
    }
}

#[derive(Bundle)]
pub struct UnitBundle {
    pub border_size: BorderSize,
    pub collider: Collider,
    pub damping: Damping,
    pub external_impulse: ExternalImpulse,
    pub locked_axis: LockedAxes,
    pub mass_properties: ColliderMassProperties, // TODO: remove
    pub name: Name,
    pub rigid_body: RigidBody,
    pub scene_root: SceneRoot, // TODO: uncomment
    pub size: pf_comps::RtsObjSize,
    pub speed: Speed,
    pub transform: Transform,
    pub transform_global: GlobalTransform,
    pub unit: Unit,
    // pub mesh: Mesh3d,
    // pub material: MeshMaterial3d<StandardMaterial>, // TODO: remove
}

impl UnitBundle {
    pub fn new(
        border_size: Vec2,
        speed: f32,
        name: String,
        scene: Handle<Scene>,
        size: Vec3,
        // mesh: Mesh3d,                               // TODO: remove
        // material: MeshMaterial3d<StandardMaterial>, // TODO: remove
        transform: Transform,
        unit: Unit,
    ) -> Self {
        // let scale = 1.55;
        Self {
            border_size: BorderSize(border_size),
            collider: Collider::cuboid(size.x, size.y, size.z), // TODO: uncomment
            // collider: Collider::cuboid(size.x * scale, size.y * scale, size.z * scale), // TODO: uncomment

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
            size: pf_comps::RtsObjSize(Vec3::new(size.x * 2.0, size.y * 2.0, size.z * 2.0)), // TODO: uncomment
            speed: Speed(speed),
            transform,
            transform_global: GlobalTransform::default(),
            unit,
            // mesh,     // TODO: remove
            // material, // TODO: remove
        }
    }
}
