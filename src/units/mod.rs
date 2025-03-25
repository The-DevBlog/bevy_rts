use bevy::prelude::*;

use crate::{
    components::structures::Structure,
    resources::{StructuresBuilt, UnlockedUnits},
    structures::mark_structure_built,
};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mark_available_units.after(mark_structure_built));
    }
}

fn mark_available_units(
    q_structures: Query<&Structure, Added<Structure>>,
    structures_built: Res<StructuresBuilt>,
    mut available_units: ResMut<UnlockedUnits>,
) {
    for structure in q_structures.iter() {
        if structures_built.vehicle_depot > 0 {
            available_units.tank_gen1 = true;
            available_units.tank_gen2 = true; // TODO: requrie research eventually
        }

        if structures_built.barracks > 0 {
            available_units.rifleman = true;
        }
    }
}
