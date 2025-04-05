use std::time::Duration;

use bevy::prelude::*;

use crate::{
    asset_manager::{audio::MyAudio, models::MyModels},
    units::{components::UnitType, events::QueueVehicleEv},
};

use super::{components::*, events::BuildVehicleEv};

pub struct VehicleDepotPlugin;

impl Plugin for VehicleDepotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildQueue>()
            .add_observer(queue_vehicle);
    }
}

#[derive(Resource, Default)]
struct BuildQueue(Vec<(UnitType, Timer)>);

fn queue_vehicle(
    trigger: Trigger<QueueVehicleEv>,
    mut cmds: Commands,
    q_structure: Query<&Transform, With<PrimaryVehicleDepot>>,
    my_models: Res<MyModels>,
    audio: Res<bevy_kira_audio::Audio>,
    my_audio: Res<MyAudio>,
    mut build_queue: ResMut<BuildQueue>,
) {
    let Ok(structure_trans) = q_structure.get_single() else {
        return;
    };

    // Set spawn offsets.
    let offset_distance = -50.0; // Distance in front of the depot (assumes front is -Z)
    let vertical_offset = 2.4; // Fixed vertical placement

    // let unit = trigger.0;
    // let timer = Timer::new(Duration::from_secs(unit.build_time()), TimerMode::Once);
    // build_queue.0.push((unit, timer));

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

fn build_vehicle(
    trigger: Trigger<BuildVehicleEv>,
    mut cmds: Commands,
    q_structure: Query<(&Transform, &StructureType), With<PrimaryStructure>>,
    my_models: Res<MyModels>,
    audio: Res<bevy_kira_audio::Audio>,
    my_audio: Res<MyAudio>,
    mut build_queue: ResMut<BuildQueue>,
) {
    // Set spawn offsets.
    // let offset_distance = -50.0; // Distance in front of the depot (assumes front is -Z)
    // let vertical_offset = 2.4; // Fixed vertical placement

    // for (structure_trans, structure_type) in q_structure.iter() {
    //     // Only build vehicle for VehicleDepot structures.
    //     if *structure_type != StructureType::VehicleDepot {
    //         continue;
    //     }

    //     // Calculate the depot's forward direction (assuming front is -Z).
    //     let forward = structure_trans.rotation * Vec3::new(0.0, 0.0, -1.0);

    //     // Compute the spawn location, overriding the Y axis with vertical_offset.
    //     let mut spawn_location = structure_trans.translation + forward * offset_distance;
    //     spawn_location.y = vertical_offset;

    //     // Create the transform for the vehicle: same as depot's rotation, but rotated 180°.
    //     let vehicle_transform = Transform {
    //         translation: spawn_location,
    //         rotation: structure_trans.rotation * Quat::from_rotation_y(std::f32::consts::PI),
    //         ..Default::default()
    //     };

    //     // Build the unit and spawn it.
    //     let unit = trigger
    //         .0
    //         .build(vehicle_transform, &my_models, &audio, &my_audio);
    //     cmds.spawn(unit);

    //     return;
    // }
}
