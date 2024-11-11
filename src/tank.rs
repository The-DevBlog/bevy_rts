use crate::{components::*, resources::*, *};
use bevy_mod_billboard::*;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::ExternalImpulse;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::events as pf_events;
use bevy_rts_pathfinding::resources as pf_res;
use bevy_rts_pathfinding::utils as pf_utils;
use events::SetUnitDestinationEv;

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tanks)
            .add_systems(Update, move_unit)
            .observe(set_unit_destination);
    }
}

fn spawn_tanks(mut cmds: Commands, assets: Res<AssetServer>, my_assets: Res<MyAssets>) {
    let initial_pos = Vec3::new(0.0, 0.0, 0.0);
    let offset = Vec3::new(20.0, 0.0, 20.0);
    let grid_size = (TANK_COUNT as f32).sqrt().ceil() as usize;

    let create_tank = |row: usize, col: usize| {
        let pos = initial_pos + Vec3::new(offset.x * row as f32, 2.0, offset.z * col as f32);
        (UnitBundle::new(
            "Tank".to_string(),
            TANK_SPEED * SPEED_QUANTIFIER,
            Vec3::new(4., 2., 6.),
            assets.load("tank_tan.glb#Scene0"),
            pos,
        ),)
    };

    let select_border = || {
        (
            BillboardTextureBundle {
                texture: BillboardTextureHandle(my_assets.select_border.clone()),
                billboard_depth: BillboardDepth(false),
                ..default()
            },
            UnitBorderBoxImg::new(15.0, 15.0),
            Name::new("Border Select"),
        )
    };

    let mut count = 0;
    for row in 0..grid_size {
        for col in 0..grid_size {
            if count >= TANK_COUNT {
                break;
            }
            cmds.spawn(create_tank(row, col)).with_children(|parent| {
                parent.spawn(select_border());
            });
            count += 1;
        }
    }
}

pub fn set_unit_destination(
    _trigger: Trigger<SetUnitDestinationEv>,
    mouse_coords: ResMut<MouseCoords>,
    mut unit_q: Query<Entity, With<pf_comps::Selected>>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mut cmds: Commands,
) {
    if !mouse_coords.in_bounds() {
        return;
    }

    let (cam, cam_trans) = cam_q.single();
    let hit = utils::cast_ray(rapier_context, &cam, &cam_trans, mouse_coords.viewport);

    if let Some(_) = hit {
        return;
    }

    for unit_entity in unit_q.iter_mut() {
        cmds.entity(unit_entity).insert(Destination);
    }

    cmds.trigger(pf_events::SetTargetCellEv);
}

// TODO: Cleanup this method
fn move_unit(
    mut flowfield_q: Query<&mut pf_comps::FlowField>,
    mut unit_q: Query<(&mut ExternalImpulse, &Transform, &Speed), With<Destination>>,
    grid: Res<pf_res::Grid>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    let rotation_magnitude = 5.0; // Set this to control the constant rotation speed
    let movement_threshold = 15.0_f32.to_radians();

    for mut flowfield in flowfield_q.iter_mut() {
        let mut units_to_remove = Vec::new();

        for &unit_entity in flowfield.entities.iter() {
            // Retrieve the unit's ExternalImpulse, Speed, and Transform
            if let Ok((mut external_impulse, unit_transform, speed)) = unit_q.get_mut(unit_entity) {
                let (row, column) = pf_utils::get_cell(&grid, &unit_transform.translation);

                // Check if the unit has reached the destination cell
                if (row as usize, column as usize) == flowfield.destination {
                    units_to_remove.push(unit_entity);
                    continue;
                }

                let flow_vector = flowfield.cells[row as usize][column as usize].flow_vector;
                if flow_vector == Vec3::ZERO {
                    continue;
                }

                // Normalize the flow vector to get the movement direction
                let desired_direction = flow_vector.normalize_or_zero();

                let forward = unit_transform.forward();
                let angle_difference = forward.angle_between(desired_direction);

                // Determine the rotation axis
                let rotation_axis = forward.cross(desired_direction).normalize_or_zero();

                // Apply rotation if the unit is not yet within 15 degrees of the desired direction
                if angle_difference > movement_threshold {
                    let torque_impulse = rotation_axis * rotation_magnitude * delta_time * 20.0;
                    external_impulse.torque_impulse += torque_impulse;
                    continue;
                }

                // Apply movement only when facing within the threshold
                let torque_impulse = rotation_axis * rotation_magnitude * delta_time * 20.0;
                external_impulse.torque_impulse += torque_impulse;

                let movement_impulse = desired_direction * speed.0 * delta_time;
                external_impulse.impulse += movement_impulse;
            }
        }

        // Remove units that have reached the target or are invalid
        flowfield.entities.retain(|e| !units_to_remove.contains(e));
    }
}
