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
            .add_systems(Update, build_vehicle_timer)
            .add_observer(obs_queue_vehicle)
            .add_observer(obs_build_vehicle);
    }
}

#[derive(Resource, Default)]
struct BuildQueue(Vec<(UnitType, Timer)>);

fn build_vehicle_timer(mut cmds: Commands, mut build_queue: ResMut<BuildQueue>, time: Res<Time>) {
    if let Some((unit_type, timer)) = build_queue.0.first_mut() {
        if timer.tick(time.delta()).just_finished() {
            cmds.trigger(BuildVehicleEv(*unit_type));
            build_queue.0.remove(0);
        }
    }
}

fn obs_queue_vehicle(trigger: Trigger<QueueVehicleEv>, mut build_queue: ResMut<BuildQueue>) {
    let unit = trigger.0;
    let timer = Timer::new(Duration::from_secs(unit.build_time()), TimerMode::Once);
    build_queue.0.push((unit, timer));
}

fn obs_build_vehicle(
    trigger: Trigger<BuildVehicleEv>,
    mut cmds: Commands,
    q_structure: Query<&Transform, With<PrimaryVehicleDepot>>,
    my_models: Res<MyModels>,
    audio: Res<bevy_kira_audio::Audio>,
    my_audio: Res<MyAudio>,
) {
    let Ok(structure_trans) = q_structure.get_single() else {
        return;
    };

    // Set spawn offsets.
    let z_offset = -10.0; // Distance in front of the depot (assumes front is -Z)
    let vertical_offset = 2.4; // Fixed vertical placement

    // Calculate the depot's forward direction (assuming front is -Z).
    let forward = structure_trans.rotation * Vec3::new(0.0, 0.0, -1.0);

    // Compute the spawn location, overriding the Y axis with vertical_offset.
    let mut spawn_location = structure_trans.translation + forward * z_offset;
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
}
