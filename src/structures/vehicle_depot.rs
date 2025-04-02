use bevy::prelude::*;

use crate::{
    asset_manager::{audio::MyAudio, models::MyModels},
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
    mut cmds: Commands,
    q_structure: Query<(&Transform, &StructureType), With<PrimaryStructure>>,
    my_models: Res<MyModels>,
    audio: Res<bevy_kira_audio::Audio>,
    my_audio: Res<MyAudio>,
) {
    // Set spawn offsets.
    let offset_distance = -50.0; // Distance in front of the depot (assumes front is -Z)
    let vertical_offset = 2.4; // Fixed vertical placement

    for (structure_trans, structure_type) in q_structure.iter() {
        // Only build vehicle for VehicleDepot structures.
        if *structure_type != StructureType::VehicleDepot {
            continue;
        }

        // Calculate the depot's forward direction (assuming front is -Z).
        let forward = structure_trans.rotation * Vec3::new(0.0, 0.0, -1.0);

        // Compute the spawn location, overriding the Y axis with vertical_offset.
        let mut spawn_location = structure_trans.translation + forward * offset_distance;
        spawn_location.y = vertical_offset;

        // Create the transform for the vehicle: same as depot's rotation, but rotated 180°.
        let vehicle_transform = Transform {
            translation: spawn_location,
            rotation: structure_trans.rotation * Quat::from_rotation_y(std::f32::consts::PI),
            ..Default::default()
        };

        // Build the unit and spawn it.
        let unit = trigger
            .0
            .build(vehicle_transform, &my_models, &audio, &my_audio);
        cmds.spawn(unit);

        return;
    }
}
