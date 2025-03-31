use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rts_pathfinding::components as pf_comps;
use strum_macros::EnumIter;

use super::{structures::StructureType, BorderSize};
use crate::{
    resources::MyAssets, tank::BORDER_SIZE, SPEED_RIFELMAN, SPEED_TANK_GEN_1, SPEED_TANK_GEN_2,
};

const TANK_GEN1_SIZE: Vec3 = Vec3::new(6.5, 3.1, 10.75);
const TANK_GEN2_SIZE: Vec3 = Vec3::new(7.5, 3.1, 13.0);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct SelectedUnit;

#[derive(Component)]
pub struct SelectionBox;

#[derive(Component, Default)]
pub struct IsMoving(pub bool);

#[derive(Component)]
#[require(pf_comps::RtsObj, IsMoving, Velocity)]
pub struct Unit;

#[derive(Component, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
#[require(pf_comps::RtsObj, IsMoving, Velocity)]
pub enum UnitType {
    Rifleman,
    TankGen1,
    TankGen2,
}

impl UnitType {
    pub fn source(&self) -> StructureType {
        match self {
            UnitType::Rifleman => StructureType::Barracks,
            UnitType::TankGen1 => StructureType::VehicleDepot,
            UnitType::TankGen2 => StructureType::VehicleDepot,
        }
    }

    pub fn hp(&self) -> i32 {
        match self {
            UnitType::Rifleman => 20,
            UnitType::TankGen1 => 100,
            UnitType::TankGen2 => 200,
        }
    }

    pub fn speed(&self) -> f32 {
        match self {
            UnitType::Rifleman => SPEED_RIFELMAN,
            UnitType::TankGen1 => SPEED_TANK_GEN_1,
            UnitType::TankGen2 => SPEED_TANK_GEN_2,
        }
    }

    pub fn dmg(&self) -> i32 {
        match self {
            UnitType::Rifleman => 1,
            UnitType::TankGen1 => 10,
            UnitType::TankGen2 => 20,
        }
    }

    pub fn build_time(&self) -> i32 {
        match self {
            UnitType::Rifleman => 1,
            UnitType::TankGen1 => 5,
            UnitType::TankGen2 => 10,
        }
    }

    pub fn cost(&self) -> i32 {
        match self {
            UnitType::Rifleman => 50,
            UnitType::TankGen1 => 500,
            UnitType::TankGen2 => 800,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            UnitType::Rifleman => "Rifleman".to_string(),
            UnitType::TankGen1 => "Tank Gen I".to_string(),
            UnitType::TankGen2 => "Tank Gen II".to_string(),
        }
    }

    pub fn img(&self, my_assets: &Res<MyAssets>) -> Handle<Image> {
        match self {
            UnitType::Rifleman => my_assets.imgs.unit_rifleman.clone(),
            UnitType::TankGen1 => my_assets.imgs.unit_tank_gen1.clone(),
            UnitType::TankGen2 => my_assets.imgs.unit_tank_gen2.clone(),
        }
    }

    fn model(&self, my_assets: &MyAssets) -> Handle<Scene> {
        match self {
            UnitType::Rifleman => my_assets.models.rifleman.clone(),
            UnitType::TankGen1 => my_assets.models.tank_gen1.clone(),
            UnitType::TankGen2 => my_assets.models.tank_gen2.clone(),
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            UnitType::Rifleman => Vec3::new(2.0, 2.0, 2.0),
            UnitType::TankGen1 => TANK_GEN1_SIZE,
            UnitType::TankGen2 => TANK_GEN2_SIZE,
        }
    }

    pub fn build(&self, transform: Transform, my_assets: &Res<MyAssets>) -> UnitBundle {
        let unit_bundle = UnitBundle::new(
            BORDER_SIZE,
            self.to_string(),
            self.model(&my_assets),
            self.size(),
            transform,
            *self,
        );

        unit_bundle
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
    pub unit_type: UnitType,
    pub unit: Unit,
    // pub mesh: Mesh3d,
    // pub material: MeshMaterial3d<StandardMaterial>, // TODO: remove
}

impl UnitBundle {
    pub fn new(
        border_size: Vec2,
        name: String,
        scene: Handle<Scene>,
        size: Vec3,
        // mesh: Mesh3d,                               // TODO: remove
        // material: MeshMaterial3d<StandardMaterial>, // TODO: remove
        transform: Transform,
        unit_type: UnitType,
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
            speed: Speed(unit_type.speed()),
            transform,
            transform_global: GlobalTransform::default(),
            unit_type: unit_type,
            unit: Unit,
            // mesh,     // TODO: remove
            // material, // TODO: remove
        }
    }
}
