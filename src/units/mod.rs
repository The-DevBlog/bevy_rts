use bevy::prelude::*;

use crate::{
    components::structures::Structure,
    resources::{AvailableUnits, StructuresBuilt},
};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mark_available_units, p));
    }
}

fn mark_available_units(
    q_structures: Query<&Structure, Added<Structure>>,
    structures_built: Res<StructuresBuilt>,
    mut available_units: ResMut<AvailableUnits>,
) {
    for structure in q_structures.iter() {
        if structures_built.vehicle_depot > 0 {
            available_units.tank_gen1 = true;
            available_units.tank_gen2 = true;
        }
    }
}

fn p(available_units: Res<AvailableUnits>) {
    println!("{:?}", available_units);
}
