use bevy::prelude::*;
use bevy_rts_pathfinding::components::{self as pf_comps};

use crate::resources::MyAssets;

#[derive(Component)]
pub struct CmdInterfaceCtr;

#[derive(Component)]
pub struct MiniMapCtr;

#[derive(Component)]
pub struct IconsCtr;

#[derive(Component)]
pub struct BuildColumnsCtr;

#[derive(Component)]
#[require(Button)]
pub struct BuildActionBtn;

#[derive(Component, Clone, Copy)]
pub struct Structure(pub StructureType);

#[derive(Clone, Copy)]
pub enum StructureType {
    Turret,
    Barracks,
    VehicleDepot,
    Black,
    White,
}

impl StructureType {
    pub fn to_string(&self) -> &str {
        match self {
            StructureType::Turret => "Turret",
            StructureType::Barracks => "Barracks",
            StructureType::VehicleDepot => "Vehicle Depot",
            StructureType::Black => "Black",
            StructureType::White => "White",
        }
    }

    pub fn img(&self, my_assets: &Res<MyAssets>) -> Handle<Image> {
        match self {
            StructureType::Turret => my_assets.images.structure_turret.clone(),
            StructureType::Barracks => my_assets.images.structure_barracks.clone(),
            StructureType::VehicleDepot => my_assets.images.structure_barracks.clone(),
            StructureType::Black => my_assets.images.structure_turret.clone(),
            StructureType::White => my_assets.images.structure_turret.clone(),
        }
    }
}

impl Structure {
    pub fn build(&self, my_assets: Res<MyAssets>) -> (SceneRoot, pf_comps::RtsObjSize) {
        match self.0 {
            StructureType::Turret => {
                let size = Vec3::new(10.0, 0.75, 10.0);
                (
                    SceneRoot(my_assets.models.turret.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::Barracks => {
                let size = Vec3::new(40.0, 16.0, 30.0);
                (
                    SceneRoot(my_assets.models.barracks.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::VehicleDepot => {
                let size = Vec3::new(30.0, 25.0, 40.0);
                (
                    SceneRoot(my_assets.models.barracks.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::Black => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    SceneRoot(my_assets.models.barracks.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::White => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    SceneRoot(my_assets.models.barracks.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
        }
    }
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct BuildStructurePlaceholder;
