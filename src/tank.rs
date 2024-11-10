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
            assets.load("tank.glb#Scene0"),
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

fn move_unit(
    mut flowfield_q: Query<&mut pf_comps::FlowField>,
    mut unit_q: Query<(&mut ExternalImpulse, &mut Transform, &Speed), With<Destination>>,
    grid: Res<pf_res::Grid>,
    time: Res<Time>,
) {
    for mut flowfield in flowfield_q.iter_mut() {
        let mut units_to_remove = Vec::new();

        for &unit_entity in flowfield.entities.iter() {
            // Get the unit's ExternalImpulse and Transform
            if let Ok((mut external_impulse, mut unit_transform, speed)) =
                unit_q.get_mut(unit_entity)
            {
                let (row, column) = pf_utils::get_cell(&grid, &unit_transform.translation);

                // Check if the unit has reached the destination cell
                if (row as usize, column as usize) == flowfield.destination {
                    units_to_remove.push(unit_entity);
                    continue;
                }

                // Get the flow vector from the flow field's cells
                let flow_vector = flowfield.cells[row as usize][column as usize].flow_vector;
                if flow_vector == Vec3::ZERO {
                    continue;
                }

                // Apply external impulse along the flow vector
                let impulse = flow_vector.normalize_or_zero() * speed.0 * time.delta_seconds();

                external_impulse.impulse += impulse;
                rotate_towards(&mut unit_transform, flow_vector);
            }
        }

        // Remove units that have reached the target or are invalid
        flowfield.entities.retain(|e| !units_to_remove.contains(e));
    }
}

pub fn rotate_towards(trans: &mut Transform, direction: Vec3) {
    let target_yaw = direction.x.atan2(direction.z);
    trans.rotation = Quat::from_rotation_y(target_yaw);
}
