use bevy::prelude::*;

use crate::{
    components::structures::{PrimaryStructure, StructureType},
    resources::MyAssets,
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
    mut cmds: Commands,
    q_structure: Query<(&Transform, &StructureType), With<PrimaryStructure>>,
    my_assets: Res<MyAssets>,
) {
    for (structure_trans, structure_type) in q_structure.iter() {
        if *structure_type != StructureType::VehicleDepot {
            continue;
        }

        // Define how far in front of the depot you want the vehicle to spawn.
        let offset_distance = -50.0;
        // Optionally, you can also add a vertical offset if needed.
        let vertical_offset = 2.4;

        // Calculate the forward direction relative to the depot's rotation.
        // Here, we assume the depot's "front" is the negative Z direction.
        let forward = structure_trans.rotation * Vec3::new(0.0, 0.0, -1.0);

        // Compute the spawn location.
        let mut spawn_location = structure_trans.translation + forward * offset_distance;
        spawn_location.y = vertical_offset;

        let unit = trigger.0.build(spawn_location, &my_assets);
        cmds.spawn(unit);
    }
}
