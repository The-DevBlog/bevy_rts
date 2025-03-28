use bevy::prelude::*;

mod vehicle_depot;

use vehicle_depot::VehicleDepotPlugin;

use crate::{
    components::structures::{Structure, StructureType},
    resources::StructuresBuilt,
};

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VehicleDepotPlugin)
            .add_systems(Update, mark_structure_built);
    }
}

// modifies the 'StructuresBuilt' resource, whenever a structure is placed or removed (destroyed)
pub fn mark_structure_built(
    mut structures_built: ResMut<StructuresBuilt>,
    q_structure_added: Query<&StructureType, Added<Structure>>,
) {
    for structure in q_structure_added.iter() {
        match structure {
            StructureType::Cannon => structures_built.cannon += 1,
            StructureType::Barracks => structures_built.barracks += 1,
            StructureType::VehicleDepot => structures_built.vehicle_depot += 1,
            StructureType::ResearchCenter => structures_built.research_center += 1,
            StructureType::SatelliteDish => structures_built.satellite_dish += 1,
        }
    }
}
