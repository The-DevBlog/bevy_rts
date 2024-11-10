use crate::{components::*, resources::*, *};
use bevy_mod_billboard::*;
use bevy_rapier3d::na::iter;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::ExternalImpulse;
use bevy_rapier3d::prelude::Sensor;
use bevy_rts_pathfinding::components as pf_comps;
use bevy_rts_pathfinding::events as pf_events;
use bevy_rts_pathfinding::resources as pf_res;
use bevy_rts_pathfinding::utils as pf_utils;
use events::SetUnitDestinationEv;

use bevy::prelude::*;
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
        (
            UnitBundle::new(
                "Tank".to_string(),
                TANK_SPEED * SPEED_QUANTIFIER,
                Vec3::new(4., 2., 6.),
                assets.load("tank.glb#Scene0"),
                pos,
            ),
            // Unit,
        )
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
    grid: Res<pf_res::Grid>,
) {
    let (cam, cam_trans) = cam_q.single();
    let hit = utils::cast_ray(rapier_context, &cam, &cam_trans, mouse_coords.viewport);

    // return if selecting another object (select another unit for example)
    if let Some(_) = hit {
        return;
    }

    let cell = pf_utils::get_cell(&grid, &mouse_coords.world);
    for unit_entity in unit_q.iter_mut() {
        cmds.entity(unit_entity)
            .insert(Destination::new(cell.0 as usize, cell.1 as usize));
    }

    cmds.trigger(pf_events::SetTargetCellEv);
}

pub fn rotate_towards(trans: &mut Transform, direction: Vec3) {
    let target_yaw = direction.x.atan2(direction.z);
    trans.rotation = Quat::from_rotation_y(target_yaw);
}

fn move_unit(
    mut flowfield_q: Query<(Entity, &mut pf_comps::FlowField)>,
    mut unit_q: Query<(&mut ExternalImpulse, &GlobalTransform, &Speed), With<Destination>>,
    grid: Res<pf_res::Grid>,
    time: Res<Time>,
    mut cmds: Commands,
) {
    for (flowfield_entity, mut flowfield) in flowfield_q.iter_mut() {
        let mut units_to_remove = Vec::new();

        for &unit_entity in flowfield.entities.iter() {
            // Get the unit's ExternalImpulse and Transform
            if let Ok((mut external_impulse, global_transform, speed)) = unit_q.get_mut(unit_entity)
            {
                // Get unit's position
                let unit_pos = global_transform.translation();

                // Map unit's position to grid cell indices
                let (cell_x, cell_z) = pf_utils::get_cell(&grid, &unit_pos);
                // Get the flow vector from the flow field's cells
                let flow_vector = flowfield.cells[cell_x as usize][cell_z as usize].flow_vector;
                // println!("row, {}, col {}", cell_x, cell_z);
                println!("flow vec: {:?}", flow_vector);
                if flow_vector == Vec3::ZERO {
                    // No movement vector; unit might be at the target or blocked
                    continue;
                }

                // Apply external impulse along the flow vector
                let impulse = flow_vector.normalize_or_zero() * speed.0 * time.delta_seconds();

                external_impulse.impulse += impulse;

                // Check if the unit has reached the target
                let target_cell =
                    &flowfield.cells[flowfield.destination.0][flowfield.destination.1];
                let target_pos = target_cell.position;
                if unit_pos.distance_squared(target_pos) < 0.1 * 0.1 {
                    // Unit has reached the target
                    units_to_remove.push(unit_entity);
                    // Optionally, perform arrival logic here
                }
                // }
            } else {
                // Unit entity does not exist or lacks required components
                // Remove it from flowfield.entities
                units_to_remove.push(unit_entity);
            }
        }

        // Remove units that have reached the target or are invalid
        flowfield.entities.retain(|e| !units_to_remove.contains(e));

        // Optionally, despawn the flow field if no units are left
        if flowfield.entities.is_empty() {
            cmds.entity(flowfield_entity).despawn();
        }
    }
}
