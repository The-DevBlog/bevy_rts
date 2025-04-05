use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalImpulse;

use crate::{
    asset_manager::{audio::MyAudio, models::MyModels},
    units::{components::UnitType, events::QueueVehicleEv},
};

use super::{components::*, events::BuildVehicleEv};

pub struct VehicleDepotPlugin;

impl Plugin for VehicleDepotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildQueue>()
            .add_systems(Update, (build_vehicle_timer, move_vehicle_from_garage))
            .add_observer(obs_queue_vehicle)
            .add_observer(obs_build_vehicle);
    }
}

#[derive(Resource, Default)]
struct BuildQueue(Vec<(UnitType, Timer)>);

// move the unit from the garage
#[derive(Component)]
struct StartPosition(Vec3);

#[derive(Component)]
struct NewUnit;

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

    let forward = structure_trans.rotation * Vec3::new(-10.0, 2.4, -12.0); // TODO: Fix this hardcoded value, especiialy the Y axis
    let spawn_location = structure_trans.translation + forward;

    // Create the transform for the vehicle: same as depot's rotation
    let vehicle_transform = Transform {
        translation: spawn_location,
        rotation: structure_trans.rotation * Quat::from_rotation_y(std::f32::consts::PI),
        ..Default::default()
    };

    let unit = trigger
        .0
        .build(vehicle_transform, &my_models, &audio, &my_audio);

    cmds.spawn((unit, NewUnit, StartPosition(vehicle_transform.translation)));
}

fn move_vehicle_from_garage(
    mut cmds: Commands,
    mut q_new_unit: Query<
        (Entity, &mut ExternalImpulse, &Transform, &StartPosition),
        With<NewUnit>,
    >,
    time: Res<Time>,
) {
    for (entity, mut ext_impulse, transform, garage) in q_new_unit.iter_mut() {
        let forward = transform.rotation * Vec3::new(0.0, 0.0, -1.0);
        ext_impulse.impulse += 300.0 * forward * time.delta_secs();

        let distance_traveled = transform.translation.distance(garage.0);
        if distance_traveled >= 50.0 {
            cmds.entity(entity).remove::<NewUnit>();
            cmds.entity(entity).remove::<StartPosition>();
        }
    }
}
