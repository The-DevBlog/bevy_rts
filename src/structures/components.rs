use bevy::prelude::*;
use bevy_pathfinding::components::{self as pf_comps};
use bevy_rapier3d::prelude::*;
use strum_macros::EnumIter;

use crate::{
    asset_manager::{imgs::MyImgs, models::MyModels},
    units::components::BorderSize,
};

#[derive(Component)]
pub struct PrimaryStructureTxt;

#[derive(Component)]
pub struct SelectedStructure;

// Hack. This is used to prevent a newly placed structure from automatically being selected
#[derive(Component)]
pub struct NewlyPlacedStructure;

#[derive(Component)]
pub struct Structure;

#[derive(Component)]
pub struct PrimaryStructure;

#[derive(Component)]
pub struct PrimaryVehicleDepot;

#[derive(Component)]
pub struct PrimaryBarracks;

#[derive(Component, Clone, Copy, EnumIter, PartialEq)]
pub enum StructureType {
    Cannon,
    Barracks,
    VehicleDepot,
    ResearchCenter,
    SatelliteDish,
}

impl StructureType {
    pub fn select_border(&self) -> BorderSize {
        match self {
            StructureType::Cannon => BorderSize(Vec2::new(40.0, 40.0)),
            StructureType::Barracks => BorderSize(Vec2::new(75.0, 75.0)),
            StructureType::VehicleDepot => BorderSize(Vec2::new(140.0, 100.0)),
            StructureType::ResearchCenter => BorderSize(Vec2::new(100.0, 100.0)),
            StructureType::SatelliteDish => BorderSize(Vec2::new(75.0, 90.0)),
        }
    }

    pub fn build_time(&self) -> u64 {
        match self {
            StructureType::Cannon => 5,
            StructureType::Barracks => 10,
            StructureType::VehicleDepot => 15,
            StructureType::ResearchCenter => 20,
            StructureType::SatelliteDish => 25,
        }
    }

    pub fn cost(&self) -> i32 {
        match self {
            StructureType::Cannon => 500,
            StructureType::Barracks => 500,
            StructureType::VehicleDepot => 2000,
            StructureType::ResearchCenter => 1500,
            StructureType::SatelliteDish => 1000,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StructureType::Cannon => "Cannon".to_string(),
            StructureType::Barracks => "Barracks".to_string(),
            StructureType::VehicleDepot => "Vehicle Depot".to_string(),
            StructureType::ResearchCenter => "Research Center".to_string(),
            StructureType::SatelliteDish => "Satellite Dish".to_string(),
        }
    }

    pub fn img(&self, my_imgs: &Res<MyImgs>) -> Handle<Image> {
        match self {
            StructureType::Cannon => my_imgs.structure_cannon.clone(),
            StructureType::Barracks => my_imgs.structure_barracks.clone(),
            StructureType::VehicleDepot => my_imgs.structure_vehicle_depot.clone(),
            StructureType::ResearchCenter => my_imgs.structure_research_center.clone(),
            StructureType::SatelliteDish => my_imgs.structure_satellite_dish.clone(),
        }
    }

    fn model(&self, my_models: &MyModels) -> Handle<Scene> {
        match self {
            StructureType::Cannon => my_models.cannon.clone(),
            StructureType::Barracks => my_models.barracks.clone(),
            StructureType::VehicleDepot => my_models.vehicle_depot.clone(),
            StructureType::ResearchCenter => my_models.research_center.clone(),
            StructureType::SatelliteDish => my_models.satellite_dish.clone(),
        }
    }

    pub fn place(
        &self,
        placeholder_ent: Entity,
        my_models: &MyModels,
        scene: &mut SceneRoot,
        rb: &mut RigidBody,
        cmds: &mut Commands,
    ) {
        *rb = RigidBody::Fixed;

        scene.0 = self.model(my_models);

        cmds.entity(placeholder_ent)
            .remove::<(ActiveEvents, Sensor, StructurePlaceholder)>()
            .insert((
                Structure,
                self.select_border(),
                NewlyPlacedStructure,
                Name::new(self.to_string()),
            ));
    }

    pub fn invalid_placement(&self, my_models: &MyModels, scene: &mut SceneRoot) {
        match self {
            StructureType::Cannon => scene.0 = my_models.placeholders.cannon_invalid.clone(),
            StructureType::Barracks => scene.0 = my_models.placeholders.barracks_invalid.clone(),
            StructureType::VehicleDepot => {
                scene.0 = my_models.placeholders.vehicle_depot_invalid.clone()
            }
            StructureType::ResearchCenter => {
                scene.0 = my_models.placeholders.research_center_invalid.clone()
            }
            StructureType::SatelliteDish => {
                scene.0 = my_models.placeholders.satellite_dish_invalid.clone()
            }
        }
    }

    pub fn valid_placement(&self, my_models: &MyModels, scene: &mut SceneRoot) {
        match self {
            StructureType::Cannon => scene.0 = my_models.placeholders.cannon_valid.clone(),
            StructureType::Barracks => scene.0 = my_models.placeholders.barracks_valid.clone(),
            StructureType::VehicleDepot => {
                scene.0 = my_models.placeholders.vehicle_depot_valid.clone()
            }
            StructureType::ResearchCenter => {
                scene.0 = my_models.placeholders.research_center_valid.clone()
            }
            StructureType::SatelliteDish => {
                scene.0 = my_models.placeholders.satellite_dish_valid.clone()
            }
        }
    }

    pub fn build_placeholder(
        &self,
        my_models: Res<MyModels>,
    ) -> (
        SceneRoot,
        Collider,
        RigidBody,
        Sensor,
        ActiveEvents,
        StructurePlaceholder,
        pf_comps::RtsObj,
    ) {
        let size;
        let structure;
        match self {
            StructureType::Cannon => {
                size = Vec3::new(10.0, 0.75, 10.0);
                structure = SceneRoot(my_models.placeholders.cannon_valid.clone());
            }
            StructureType::Barracks => {
                size = Vec3::new(30.0, 12.0, 25.0);
                structure = SceneRoot(my_models.placeholders.barracks_valid.clone());
            }
            StructureType::VehicleDepot => {
                size = Vec3::new(60.0, 4.0, 40.0);
                structure = SceneRoot(my_models.placeholders.vehicle_depot_valid.clone());
            }
            StructureType::ResearchCenter => {
                size = Vec3::new(30.0, 18.0, 30.0);
                structure = SceneRoot(my_models.placeholders.research_center_valid.clone());
            }
            StructureType::SatelliteDish => {
                size = Vec3::new(32.0, 8.0, 32.0);
                structure = SceneRoot(my_models.placeholders.satellite_dish_valid.clone());
            }
        }

        (
            structure,
            Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            RigidBody::Dynamic,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            StructurePlaceholder::new(*self),
            pf_comps::RtsObj(Vec2::new(size.x, size.z)),
        )
    }
}

#[derive(Component)]
pub struct StructurePlaceholder {
    pub is_valid: bool,
    pub structure: StructureType,
}

impl StructurePlaceholder {
    pub fn new(structure: StructureType) -> Self {
        Self {
            is_valid: true,
            structure,
        }
    }
}
