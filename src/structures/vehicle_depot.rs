use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
use std::time::Duration;

use crate::{
    asset_manager::{
        audio::{AudioCmd, UnitAudioEv},
        models::MyModels,
    },
    cmd_interface::resources::BuildQueueCount,
    units::{components::Speed, events::QueueVehicleEv},
};

use super::{components::*, events::BuildVehicleEv, resources::VehicleBuildQueue};

pub struct VehicleDepotPlugin;

impl Plugin for VehicleDepotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (build_vehicle_timer, move_vehicle_from_garage))
            .add_observer(obs_queue_vehicle)
            .add_observer(obs_build_vehicle);
    }
}

// move the unit from the garage
#[derive(Component)]
struct NewUnit {
    start_pos: Vec3,
    timer: Timer,
}

impl NewUnit {
    fn new(start_pos: Vec3) -> Self {
        Self {
            start_pos,
            timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
        }
    }
}

fn build_vehicle_timer(
    mut cmds: Commands,
    mut build_queue: ResMut<VehicleBuildQueue>,
    time: Res<Time>,
) {
    if let Some((unit_type, timer)) = build_queue.0.first_mut() {
        if timer.tick(time.delta()).just_finished() {
            cmds.trigger(BuildVehicleEv(*unit_type));
            build_queue.0.remove(0);
        }
    }
}

fn obs_queue_vehicle(trigger: Trigger<QueueVehicleEv>, mut build_queue: ResMut<VehicleBuildQueue>) {
    let unit = trigger.0;
    let timer = Timer::new(Duration::from_secs(unit.build_time()), TimerMode::Once);
    build_queue.0.push((unit, timer));
}

fn obs_build_vehicle(
    trigger: Trigger<BuildVehicleEv>,
    mut cmds: Commands,
    q_structure: Query<&Transform, With<PrimaryVehicleDepot>>,
    my_models: Res<MyModels>,
    // audio: Res<bevy_kira_audio::Audio>,
    // my_audio: Res<MyAudio>,
    mut build_queue_count: ResMut<BuildQueueCount>,
) {
    let Ok(structure_trans) = q_structure.single() else {
        return;
    };

    let unit_type = trigger.0;

    let forward: Vec3 = structure_trans.rotation * Vec3::new(-10.0, 0.0, -5.0);
    let mut spawn_location = structure_trans.translation + forward;
    spawn_location.y = 2.0; // TODO: Fix this hardcoded value. Needs to be dynamic based on unit models height

    // Create the transform for the vehicle: same as depot's rotation
    let vehicle_transform = Transform {
        translation: spawn_location,
        rotation: structure_trans.rotation * Quat::from_rotation_y(std::f32::consts::PI),
        ..Default::default()
    };

    let unit = unit_type.build(vehicle_transform, &my_models);

    cmds.trigger(UnitAudioEv::new(AudioCmd::Ready, unit_type.clone()));
    cmds.spawn((unit, NewUnit::new(vehicle_transform.translation)));

    build_queue_count.remove(&unit_type);
}

fn move_vehicle_from_garage(
    mut cmds: Commands,
    mut q_new_unit: Query<(Entity, &mut Velocity, &Transform, &Speed, &mut NewUnit)>,
    time: Res<Time>,
) {
    for (entity, mut vel, tf, speed, mut new_unit) in q_new_unit.iter_mut() {
        new_unit.timer.tick(time.delta());

        if !new_unit.timer.finished() {
            continue;
        }

        let forward = tf.rotation * Vec3::new(0.0, 0.0, -1.0);
        vel.linvel = forward.normalize() * speed.0;

        let distance_traveled = tf.translation.distance(new_unit.start_pos);
        if distance_traveled >= 50.0 {
            vel.linvel = Vec3::ZERO;
            cmds.entity(entity).remove::<NewUnit>();
        }
    }
}
