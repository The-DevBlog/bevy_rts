use bevy::prelude::*;
use bevy_rts_pathfinding::components::{self as pf_comps};

use crate::resources::MyAssets;

#[derive(Component)]
pub struct CmdInterfaceCtr;

#[derive(Component, Clone, Copy)]
pub struct Structure(pub StructureType);

#[derive(Clone, Copy)]
pub enum StructureType {
    Cannon,
    Barracks,
    VehicleDepot,
    ResearchCenter,
    SatelliteDish,
}

impl StructureType {
    pub fn to_string(&self) -> &str {
        match self {
            StructureType::Cannon => "Cannon",
            StructureType::Barracks => "Barracks",
            StructureType::VehicleDepot => "Vehicle Depot",
            StructureType::ResearchCenter => "Research Center",
            StructureType::SatelliteDish => "Satellite Dish",
        }
    }

    pub fn img(&self, my_assets: &Res<MyAssets>) -> Handle<Image> {
        match self {
            StructureType::Cannon => my_assets.images.structure_cannon.clone(),
            StructureType::Barracks => my_assets.images.structure_barracks.clone(),
            StructureType::VehicleDepot => my_assets.images.structure_vehicle_depot.clone(),
            StructureType::ResearchCenter => my_assets.images.structure_research_center.clone(),
            StructureType::SatelliteDish => my_assets.images.structure_satellite_dish.clone(),
        }
    }
}

impl Structure {
    pub fn place(&self, my_assets: &MyAssets, scene: &mut SceneRoot) {
        match self.0 {
            StructureType::Cannon => scene.0 = my_assets.models.cannon.clone(),
            StructureType::Barracks => scene.0 = my_assets.models.barracks.clone(),
            StructureType::VehicleDepot => scene.0 = my_assets.models.vehicle_depot.clone(),
            StructureType::ResearchCenter => scene.0 = my_assets.models.research_center.clone(),
            StructureType::SatelliteDish => scene.0 = my_assets.models.satellite_dish.clone(),
        }
    }

    pub fn build(&self, my_assets: Res<MyAssets>) -> (SceneRoot, pf_comps::RtsObjSize) {
        match self.0 {
            StructureType::Cannon => {
                let size = Vec3::new(10.0, 0.75, 10.0);
                (
                    SceneRoot(my_assets.models.placeholders.cannon_valid.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::Barracks => {
                let size = Vec3::new(30.0, 12.0, 25.0);
                (
                    SceneRoot(my_assets.models.placeholders.barracks_valid.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::VehicleDepot => {
                let size = Vec3::new(60.0, 4.0, 40.0);
                (
                    SceneRoot(my_assets.models.placeholders.vehicle_depot_valid.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::ResearchCenter => {
                let size = Vec3::new(30.0, 18.0, 30.0);
                (
                    SceneRoot(my_assets.models.placeholders.research_center_valid.clone()),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::SatelliteDish => {
                let size = Vec3::new(32.0, 8.0, 32.0);
                (
                    SceneRoot(my_assets.models.placeholders.satellite_dish_valid.clone()),
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
