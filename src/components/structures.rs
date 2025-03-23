use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rts_pathfinding::components::{self as pf_comps};
use strum_macros::EnumIter;

use crate::resources::MyAssets;

#[derive(Component, Clone, Copy, EnumIter)]
pub enum Structure {
    Cannon,
    Barracks,
    VehicleDepot,
    ResearchCenter,
    SatelliteDish,
}

impl Structure {
    pub fn cost(&self) -> i32 {
        match self {
            Structure::Cannon => 500,
            Structure::Barracks => 500,
            Structure::VehicleDepot => 2000,
            Structure::ResearchCenter => 1500,
            Structure::SatelliteDish => 1000,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Structure::Cannon => "Cannon",
            Structure::Barracks => "Barracks",
            Structure::VehicleDepot => "Vehicle Depot",
            Structure::ResearchCenter => "Research Center",
            Structure::SatelliteDish => "Satellite Dish",
        }
    }

    pub fn img(&self, my_assets: &Res<MyAssets>) -> Handle<Image> {
        match self {
            Structure::Cannon => my_assets.images.structure_cannon.clone(),
            Structure::Barracks => my_assets.images.structure_barracks.clone(),
            Structure::VehicleDepot => my_assets.images.structure_vehicle_depot.clone(),
            Structure::ResearchCenter => my_assets.images.structure_research_center.clone(),
            Structure::SatelliteDish => my_assets.images.structure_satellite_dish.clone(),
        }
    }
}

impl Structure {
    pub fn place(
        &self,
        placeholder_ent: Entity,
        my_assets: &MyAssets,
        scene: &mut SceneRoot,
        rb: &mut RigidBody,
        cmds: &mut Commands,
    ) {
        *rb = RigidBody::Fixed;

        match self {
            Structure::Cannon => scene.0 = my_assets.models.cannon.clone(),
            Structure::Barracks => scene.0 = my_assets.models.barracks.clone(),
            Structure::VehicleDepot => scene.0 = my_assets.models.vehicle_depot.clone(),
            Structure::ResearchCenter => scene.0 = my_assets.models.research_center.clone(),
            Structure::SatelliteDish => scene.0 = my_assets.models.satellite_dish.clone(),
        }

        cmds.entity(placeholder_ent)
            .remove::<StructurePlaceholder>();
        cmds.entity(placeholder_ent).remove::<ActiveEvents>();
        cmds.entity(placeholder_ent).remove::<Sensor>();
        cmds.entity(placeholder_ent).insert(pf_comps::RtsObj);
    }

    pub fn invalid_placement(&self, assets: &MyAssets, scene: &mut SceneRoot) {
        match self {
            Structure::Cannon => scene.0 = assets.models.placeholders.cannon_invalid.clone(),
            Structure::Barracks => scene.0 = assets.models.placeholders.barracks_invalid.clone(),
            Structure::VehicleDepot => {
                scene.0 = assets.models.placeholders.vehicle_depot_invalid.clone()
            }
            Structure::ResearchCenter => {
                scene.0 = assets.models.placeholders.research_center_invalid.clone()
            }
            Structure::SatelliteDish => {
                scene.0 = assets.models.placeholders.satellite_dish_invalid.clone()
            }
        }
    }

    pub fn valid_placement(&self, assets: &MyAssets, scene: &mut SceneRoot) {
        match self {
            Structure::Cannon => scene.0 = assets.models.placeholders.cannon_valid.clone(),
            Structure::Barracks => scene.0 = assets.models.placeholders.barracks_valid.clone(),
            Structure::VehicleDepot => {
                scene.0 = assets.models.placeholders.vehicle_depot_valid.clone()
            }
            Structure::ResearchCenter => {
                scene.0 = assets.models.placeholders.research_center_valid.clone()
            }
            Structure::SatelliteDish => {
                scene.0 = assets.models.placeholders.satellite_dish_valid.clone()
            }
        }
    }

    pub fn build_placeholder(
        &self,
        my_assets: Res<MyAssets>,
    ) -> (
        SceneRoot,
        Collider,
        RigidBody,
        Sensor,
        ActiveEvents,
        StructurePlaceholder,
        pf_comps::RtsObjSize,
    ) {
        let size;
        let structure;
        match self {
            Structure::Cannon => {
                size = Vec3::new(10.0, 0.75, 10.0);
                structure = SceneRoot(my_assets.models.placeholders.cannon_valid.clone());
            }
            Structure::Barracks => {
                size = Vec3::new(30.0, 12.0, 25.0);
                structure = SceneRoot(my_assets.models.placeholders.barracks_valid.clone());
            }
            Structure::VehicleDepot => {
                size = Vec3::new(60.0, 4.0, 40.0);
                structure = SceneRoot(my_assets.models.placeholders.vehicle_depot_valid.clone());
            }
            Structure::ResearchCenter => {
                size = Vec3::new(30.0, 18.0, 30.0);
                structure = SceneRoot(my_assets.models.placeholders.research_center_valid.clone());
            }
            Structure::SatelliteDish => {
                size = Vec3::new(32.0, 8.0, 32.0);
                structure = SceneRoot(my_assets.models.placeholders.satellite_dish_valid.clone());
            }
        }

        (
            structure,
            Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            RigidBody::Dynamic,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            StructurePlaceholder::new(*self),
            pf_comps::RtsObjSize(size),
        )
    }
}

#[derive(Component)]
pub struct StructurePlaceholder {
    pub is_valid: bool,
    pub structure: Structure,
}

impl StructurePlaceholder {
    pub fn new(structure: Structure) -> Self {
        Self {
            is_valid: true,
            structure,
        }
    }
}
