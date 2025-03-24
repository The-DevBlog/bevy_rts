use bevy::prelude::*;

mod vehicle_depot;

use vehicle_depot::VehicleDepotPlugin;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VehicleDepotPlugin);
    }
}
