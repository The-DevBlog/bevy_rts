use bevy::prelude::*;
use events::{BuildSoldierEv, QueueVehicleEv};

use crate::cmd_interface::events::BuildUnitEv;
use crate::resources::DbgOptions;
use crate::structures::components::*;
use crate::structures::resources::StructuresBuilt;
use crate::structures::*;

pub mod components;
pub mod events;
pub mod resources;

use resources::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ResourcesPlugin)
            .add_systems(Update, mark_available_units.after(count_structures))
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
fn handle_build_unit(trigger: Trigger<BuildUnitEv>, mut cmds: Commands, dbg: Res<DbgOptions>) {
    let unit_type = trigger.0;

    dbg.print(&format!("Building unit: {}", unit_type.name()));

    match unit_type.source() {
        StructureType::Barracks => cmds.trigger(BuildSoldierEv(unit_type)),
        StructureType::VehicleDepot => cmds.trigger(QueueVehicleEv(unit_type)),
        _ => (),
    }
}
