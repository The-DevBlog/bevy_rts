use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rts_pathfinding::components as pf_comps;
use strum_macros::EnumIter;

use crate::asset_manager::audio::*;
use crate::asset_manager::imgs::MyImgs;
use crate::asset_manager::models::MyModels;
use crate::structures::components::StructureType;
use crate::tank::*;
use crate::*;

const TANK_GEN_1_SIZE: Vec3 = Vec3::new(6.5, 3.1, 10.75);
const TANK_GEN_2_SIZE: Vec3 = Vec3::new(7.5, 3.1, 13.0);

#[derive(Component, Clone)]
pub struct UnitSelectBorder(pub Entity);

#[derive(Component)]
pub struct BorderSize(pub Vec2);

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

    pub fn build_time(&self) -> u64 {
        match self {
            UnitType::Rifleman => 1,
            UnitType::TankGen1 => 2,
            UnitType::TankGen2 => 2,
        }
    }

    pub fn cost(&self) -> i32 {
        match self {
            UnitType::Rifleman => 50,
            UnitType::TankGen1 => 500,
            UnitType::TankGen2 => 800,
        }
    }

    pub fn name(&self) -> String {
        match self {
            UnitType::Rifleman => "Rifleman".to_string(),
            UnitType::TankGen1 => "Tank Gen I".to_string(),
            UnitType::TankGen2 => "Tank Gen II".to_string(),
        }
    }

    pub fn img(&self, my_imgs: &Res<MyImgs>) -> Handle<Image> {
        match self {
            UnitType::Rifleman => my_imgs.unit_rifleman.clone(),
            UnitType::TankGen1 => my_imgs.unit_tank_gen1.clone(),
            UnitType::TankGen2 => my_imgs.unit_tank_gen2.clone(),
        }
    }

    fn model(&self, my_models: &MyModels) -> Handle<Scene> {
        match self {
            UnitType::Rifleman => my_models.rifleman.clone(),
            UnitType::TankGen1 => my_models.tank_gen1.clone(),
            UnitType::TankGen2 => my_models.tank_gen2.clone(),
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            UnitType::Rifleman => Vec3::new(2.0, 2.0, 2.0), // TODO: Define rifleman size
            UnitType::TankGen1 => TANK_GEN_1_SIZE,
            UnitType::TankGen2 => TANK_GEN_2_SIZE,
        }
    }

    fn audio_emitter(
        &self,
        audio: &bevy_kira_audio::Audio,
        my_audio: &MyAudio,
    ) -> SpatialAudioEmitter {
        let audio_handles = match self {
            UnitType::Rifleman => {
                let handle = my_audio.sfx.moving_rifleman.source.clone();
                vec![audio.play(handle).looped().paused().handle()]
            }
            UnitType::TankGen1 => {
                let handle = my_audio.sfx.moving_tank_gen_1.source.clone();
                vec![audio.play(handle).looped().paused().handle()]
            }
            UnitType::TankGen2 => {
                let handle = my_audio.sfx.moving_tank_gen_2.source.clone();
                vec![audio.play(handle).looped().paused().handle()]
            }
        };

        SpatialAudioEmitter {
            instances: audio_handles,
        }
    }

    pub fn build(
        &self,
        transform: Transform,
        my_models: &Res<MyModels>,
        audio: &bevy_kira_audio::Audio,
        my_audio: &MyAudio,
    ) -> UnitBundle {
        let unit_bundle = UnitBundle::new(
            BORDER_SIZE,
            self.name(),
            self.model(&my_models),
            self.size(),
            transform,
            *self,
            self.audio_emitter(&audio, &my_audio),
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
    pub scene_root: SceneRoot,
    pub size: pf_comps::RtsObjSize,
    pub speed: Speed,
    pub transform: Transform,
    pub transform_global: GlobalTransform,
    pub unit_type: UnitType,
    pub unit: Unit,
    pub audio_emitter: SpatialAudioEmitter,
    pub spatial_audio_radius: SpatialRadius,
}

impl UnitBundle {
    fn new(
        border_size: Vec2,
        name: String,
        scene: Handle<Scene>,
        size: Vec3,
        transform: Transform,
        unit_type: UnitType,
        audio_emitter: SpatialAudioEmitter,
    ) -> Self {
        Self {
            border_size: BorderSize(border_size),
            collider: Collider::cuboid(size.x, size.y, size.z),
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
            scene_root: SceneRoot(scene),
            size: pf_comps::RtsObjSize(Vec3::new(size.x * 2.0, size.y * 2.0, size.z * 2.0)),
            speed: Speed(unit_type.speed()),
            transform,
            transform_global: GlobalTransform::default(),
            unit_type: unit_type,
            unit: Unit,
            audio_emitter,
            spatial_audio_radius: SpatialRadius { radius: 350.0 }, // TODO For some reason anything above 150 and I cant hear anything at all
        }
    }
}
