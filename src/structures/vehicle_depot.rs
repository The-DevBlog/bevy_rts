use bevy::prelude::*;

use crate::{
    components::structures::{PrimaryStructure, StructureType},
    units::events::BuildVehicle,
};

pub struct VehicleDepotPlugin;

impl Plugin for VehicleDepotPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(build_vehicle);
    }
}

fn build_vehicle(
    trigger: Trigger<BuildVehicle>,
    q_structure: Query<(&Transform, &StructureType), With<PrimaryStructure>>,
) {
    for (structure_trans, structure_type) in q_structure.iter() {
        if *structure_type != StructureType::VehicleDepot {
            continue;
        }

        let unit_type = trigger.0;
    }
}
