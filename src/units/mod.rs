use bevy::prelude::*;
use bevy_rapier3d::plugin::{DefaultRapierContext, RapierContext};
use bevy_rapier3d::prelude::Velocity;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::events as pf_events;
use bevy_rts_pathfinding::flowfield::FlowField;
use components::{IsMoving, SelectedUnit, Speed, UnitType};
use events::{QueueSolderEv, QueueVehicleEv};

use crate::cmd_interface::events::BuildUnitEv;
use crate::events::SetUnitDestinationEv;
use crate::resources::{DbgOptions, MouseCoords};
use crate::structures::components::*;
use crate::structures::resources::StructuresBuilt;
use crate::{structures::*, utils};

pub mod components;
pub mod events;
pub mod resources;

use resources::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ResourcesPlugin)
            .add_systems(
                Update,
                (
                    set_is_moving,
                    stop_movement,
                    mark_available_units.after(count_structures),
                    move_unit.run_if(any_with_component::<pf_comps::Destination>),
                ),
            )
            .add_observer(set_unit_destination)
            .add_observer(handle_build_unit);
    }
}

fn mark_available_units(
    q_structures: Query<&Structure, Added<Structure>>,
    structures_built: Res<StructuresBuilt>,
    mut available_units: ResMut<UnlockedUnits>,
) {
    for _structure in q_structures.iter() {
        if structures_built.vehicle_depot > 0 {
            available_units.tank_gen1 = true;
            available_units.tank_gen2 = true; // TODO: requrie research eventually
            available_units.artillery = true;
        }

        if structures_built.barracks > 0 {
            available_units.rifleman = true;
        }
    }
}

// this consumes the BuildUnitEv, and determines which units to build (from vehicle depot or barracks)
fn handle_build_unit(trigger: Trigger<BuildUnitEv>, mut cmds: Commands, dbg: Res<DbgOptions>) {
    let unit_type = trigger.0;

    dbg.print(&format!("Building unit: {}", unit_type.name()));

    match unit_type.source() {
        StructureType::Barracks => cmds.trigger(QueueSolderEv(unit_type)),
        StructureType::VehicleDepot => cmds.trigger(QueueVehicleEv(unit_type)),
        _ => (),
    }
}

pub fn set_unit_destination(
    _trigger: Trigger<SetUnitDestinationEv>,
    mouse_coords: ResMut<MouseCoords>,
    mut q_unit: Query<Entity, With<SelectedUnit>>,
    q_cam: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    q_rapier: Query<&RapierContext, With<DefaultRapierContext>>,
    mut cmds: Commands,
) {
    if !mouse_coords.in_bounds() {
        return;
    }

    let Ok(rapier_ctx) = q_rapier.get_single() else {
        return;
    };

    let (cam, cam_trans) = q_cam.single();
    let hit = utils::cast_ray(rapier_ctx, &cam, &cam_trans, mouse_coords.viewport);

    if let Some(_) = hit {
        return;
    }

    let mut units = Vec::new();
    for unit_entity in q_unit.iter_mut() {
        cmds.entity(unit_entity).insert(pf_comps::Destination);
        units.push(unit_entity);
    }

    cmds.trigger(pf_events::InitializeFlowFieldEv(units));
}

fn set_is_moving(mut q_is_moving: Query<(&mut IsMoving, &Velocity), With<UnitType>>) {
    for (mut is_moving, velocity) in q_is_moving.iter_mut() {
        is_moving.0 = velocity.linvel.length_squared() > 0.0001;
    }
}

fn stop_movement(
    mut q_vel: Query<&mut Velocity>,
    mut removed: RemovedComponents<pf_comps::Destination>,
) {
    let ents: Vec<Entity> = removed.read().collect();
    for ent in ents {
        if let Ok(mut vel) = q_vel.get_mut(ent) {
            vel.linvel = Vec3::ZERO;
        }
    }
}

fn move_unit(
    q_ff: Query<&FlowField>,
    mut q_units: Query<
        (
            Entity,
            &mut Transform,
            &pf_comps::Boid,
            &Speed,
            &mut Velocity,
        ),
        With<pf_comps::Destination>,
    >,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let rotation_speed = 5.0; // radians/sec

    for ff in q_ff.iter() {
        for (ent, mut tx, _boid, speed, mut vel) in q_units.iter_mut() {
            if let Some(steering) = ff.steering_map.get(&ent) {
                // ——— 1) Rotate toward steering ———
                if steering.length_squared() > 1e-6 {
                    // Compute the yaw so that “forward” (-Z) points along steering
                    let target_yaw = f32::atan2(-steering.x, -steering.z);
                    let target_rot = Quat::from_rotation_y(target_yaw);

                    // Slerp current rotation → target
                    let min_rotation = 0.1;
                    let angle_diff = tx.rotation.angle_between(target_rot);
                    if angle_diff > min_rotation {
                        let max_step = rotation_speed * dt;
                        let t = (max_step.min(angle_diff)) / angle_diff;
                        tx.rotation = tx.rotation.slerp(target_rot, t);
                    } else {
                        let t = (rotation_speed * dt).clamp(0.0, 1.0);
                        tx.rotation = tx.rotation.slerp(target_rot, t);
                    }

                    // ——— 2) Drive velocity along steering ———
                    vel.linvel = steering.normalize() * speed.0;
                } else {
                    // No steering → stop
                    vel.linvel = Vec3::ZERO;
                }
            }
        }
    }
}
