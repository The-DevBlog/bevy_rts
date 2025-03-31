use bevy::prelude::*;

use crate::components::structures::{PrimaryStructure, StructureType};

pub struct VehicleDepotPlugin;

impl Plugin for VehicleDepotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_vehicle);
    }
}

fn build_vehicle(q_structure: Query<(&Transform, &StructureType), With<PrimaryStructure>>) {
    for (structure_trans, structure_type) in q_structure.iter() {
        if *structure_type != StructureType::VehicleDepot {
            continue;
        }
    }
}
