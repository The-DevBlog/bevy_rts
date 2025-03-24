use bevy::prelude::*;

mod vehicle_depot;

use vehicle_depot::VehicleDepotPlugin;

use crate::{components::structures::Structure, resources::StructuresBuilt};

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VehicleDepotPlugin)
            .add_systems(Update, mark_structure_built);
    }
}

// modifies the 'StructuresBuilt' resource, whenever a structure is placed or removed (destroyed)
fn mark_structure_built(
    mut structures_built: ResMut<StructuresBuilt>,
    q_structure_added: Query<&Structure, Added<Structure>>,
    q_structure: Query<&Structure>,
) {
    for s in q_structure_added.iter() {}
}
