use bevy::prelude::*;

use crate::cmd_interface::events::BuildUnitEv;
use crate::components::structures::*;
use crate::resources::structures::StructuresBuilt;
use crate::resources::units::UnlockedUnits;
use crate::structures::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mark_available_units.after(count_structures))
            .add_observer(handle_build_unit);
    }
}

fn mark_available_units(
    q_structures: Query<&Structure, Added<Structure>>,
    structures_built: Res<StructuresBuilt>,
    mut available_units: ResMut<UnlockedUnits>,
) {
    for _structure in q_structures.iter() {
        if structures_built.vehicle_depot > 0 {
            available_units.tank_gen1 = true;
            available_units.tank_gen2 = true; // TODO: requrie research eventually
        }

        if structures_built.barracks > 0 {
            available_units.rifleman = true;
        }
    }
}

// this consumes the BuildUnitEv, and determines which units to build (from vehicle depot or barracks)
fn handle_build_unit(trigger: Trigger<BuildUnitEv>) {
    println!("Handle Build Unit!!!");
}
