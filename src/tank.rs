use bevy::prelude::*;
use bevy_mod_billboard::*;
use bevy_rapier3d::{plugin::RapierContext, prelude::*};
use events::SetUnitDestinationEv;
use map::{Grid, TargetCell};
use path_finding::find_path;

use crate::{components::*, resources::*, utils, *};

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tanks)
            .add_systems(Update, move_units_along_path)
            // .add_systems(Update, (move_unit::<Friendly>,))
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
            Selected(false),
            Friendly,
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
    mut friendly_q: Query<(&mut Destination, &Transform, &Selected), With<Friendly>>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
) {
    let (cam, cam_trans) = cam_q.single();
    let hit = utils::cast_ray(rapier_context, &cam, &cam_trans, mouse_coords.viewport);

    // return if selecting another object (select another unit for example)
    if let Some(_) = hit {
        return;
    }

    for (mut friendly_destination, trans, selected) in friendly_q.iter_mut() {
        if selected.0 {
            let mut destination = mouse_coords.world;
            destination.y += trans.scale.y / 2.0; // calculate for entity height
            friendly_destination.0 = Some(destination);
            // println!("Unit Moving to ({}, {})", destination.x, destination.y);
        }
    }
}

fn move_units_along_path(
    time: Res<Time>,
    mut commands: Commands,
    mut unit_q: Query<(
        Entity,
        &mut Transform,
        &mut DestinationPath,
        &Speed,
        &mut CurrentAction,
        &mut ExternalImpulse,
    )>,
) {
    for (entity, mut transform, mut path, speed, mut action, mut ext_impulse) in unit_q.iter_mut() {
        // Check if we've reached the end of the path
        if path.current_index >= path.waypoints.len() {
            // Remove the DestinationPath component
            // commands.entity(entity).remove::<DestinationPath>();
            path.current_index = 0;
            path.waypoints = Vec::default();
            action.0 = Action::None;
            continue;
        }

        // Get the current waypoint
        let cell = &path.waypoints[path.current_index];
        let target_position = Vec3::new(cell.position.x, transform.translation.y, cell.position.y);

        let distance = target_position - transform.translation;
        let distance_sq = distance.length_squared();

        if distance_sq < 0.1 * 0.1 {
            // Reached the waypoint, move to the next one
            path.current_index += 1;
            if path.current_index >= path.waypoints.len() {
                // Remove the DestinationPath component
                // commands.entity(entity).remove::<DestinationPath>();
                let index = path.current_index.clone();
                path.waypoints.remove(index);
                action.0 = Action::None;
                continue;
            }
        } else {
            // Move towards the waypoint
            let direction = Vec3::new(distance.x, 0.0, distance.z).normalize();
            rotate_towards(&mut transform, direction);

            action.0 = Action::Relocate;
            ext_impulse.impulse += direction * speed.0 * time.delta_seconds();
        }
    }
}

// fn move_unit<T: Component>(
//     mut unit_q: Query<
//         (
//             &mut CurrentAction,
//             &mut Transform,
//             &mut ExternalImpulse,
//             &Speed,
//             &mut Destination,
//         ),
//         With<T>,
//     >,
//     time: Res<Time>,
// ) {
//     for (mut action, mut trans, mut ext_impulse, speed, mut destination) in unit_q.iter_mut() {
//         // only move the object if it has a destination
//         if let Some(new_pos) = destination.0 {
//             let distance = new_pos - trans.translation;
//             let direction = Vec3::new(distance.x, 0.0, distance.z).normalize();
//             rotate_towards(&mut trans, direction);

//             if distance.length_squared() <= 5.0 {
//                 destination.0 = None;
//                 action.0 = Action::None;
//             } else {
//                 action.0 = Action::Relocate;
//                 ext_impulse.impulse += direction * speed.0 * time.delta_seconds();
//             }
//         }
//     }
// }

fn rotate_towards(trans: &mut Transform, direction: Vec3) {
    let target_yaw = direction.x.atan2(direction.z);
    trans.rotation = Quat::from_rotation_y(target_yaw);
}
