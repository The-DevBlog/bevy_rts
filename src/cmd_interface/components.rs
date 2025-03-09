use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, RigidBody, Sensor};
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
    pub fn place(
        &self,
        placeholder_ent: Entity,
        my_assets: &MyAssets,
        scene: &mut SceneRoot,
        rb: &mut RigidBody,
        cmds: &mut Commands,
    ) {
        *rb = RigidBody::Fixed;

        match self.0 {
            StructureType::Cannon => scene.0 = my_assets.models.cannon.clone(),
            StructureType::Barracks => scene.0 = my_assets.models.barracks.clone(),
            StructureType::VehicleDepot => scene.0 = my_assets.models.vehicle_depot.clone(),
            StructureType::ResearchCenter => scene.0 = my_assets.models.research_center.clone(),
            StructureType::SatelliteDish => scene.0 = my_assets.models.satellite_dish.clone(),
        }

        cmds.entity(placeholder_ent)
            .remove::<BuildStructurePlaceholder>();
        cmds.entity(placeholder_ent).remove::<ActiveEvents>();
        cmds.entity(placeholder_ent).remove::<Sensor>();
        cmds.entity(placeholder_ent).insert(pf_comps::RtsObj);
    }

    pub fn invalid_placement(&self, my_assets: &MyAssets, scene: &mut SceneRoot) {
        match self.0 {
            StructureType::Cannon => scene.0 = my_assets.models.placeholders.cannon_invalid.clone(),
            StructureType::Barracks => {
                scene.0 = my_assets.models.placeholders.barracks_invalid.clone()
            }
            StructureType::VehicleDepot => {
                scene.0 = my_assets.models.placeholders.vehicle_depot_invalid.clone()
            }
            StructureType::ResearchCenter => {
                scene.0 = my_assets
                    .models
                    .placeholders
                    .research_center_invalid
                    .clone()
            }
            StructureType::SatelliteDish => {
                scene.0 = my_assets.models.placeholders.satellite_dish_invalid.clone()
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
        BuildStructurePlaceholder,
        pf_comps::RtsObjSize,
    ) {
        let size;
        let structure;
        match self.0 {
            StructureType::Cannon => {
                size = Vec3::new(10.0, 0.75, 10.0);
                structure = SceneRoot(my_assets.models.placeholders.cannon_valid.clone());
            }
            StructureType::Barracks => {
                size = Vec3::new(30.0, 12.0, 25.0);
                structure = SceneRoot(my_assets.models.placeholders.barracks_valid.clone());
            }
            StructureType::VehicleDepot => {
                size = Vec3::new(60.0, 4.0, 40.0);
                structure = SceneRoot(my_assets.models.placeholders.vehicle_depot_valid.clone());
            }
            StructureType::ResearchCenter => {
                size = Vec3::new(30.0, 18.0, 30.0);
                structure = SceneRoot(my_assets.models.placeholders.research_center_valid.clone());
            }
            StructureType::SatelliteDish => {
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
            BuildStructurePlaceholder::default(),
            pf_comps::RtsObjSize(size),
        )
    }
}

#[derive(Component)]
pub struct Unit;

#[derive(Component, Default)]
pub struct BuildStructurePlaceholder {
    is_valid: bool,
}
