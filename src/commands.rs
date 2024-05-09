use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext, render::ColliderDebugColor};
use bevy_rts_camera::RtsCamera;

use crate::{
    map::MapBase,
    resources::GroundCoords,
    units::{Selected, Speed, TargetPos, Unit},
};

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                set_unit_destination,
                set_ground_coords,
                move_unit,
                select,
                deselect,
            ),
        );
    }
}

fn set_unit_destination(
    ground_coords: ResMut<GroundCoords>,
    mut unit_q: Query<(&mut TargetPos, &Selected), With<Unit>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Left) {
        for (mut unit_target_pos, selected) in unit_q.iter_mut() {
            if selected.0 {
                unit_target_pos.0 = Some(ground_coords.global);
                println!("Unit Moving");
            }
        }
    }
}

fn move_unit(
    mut unit_q: Query<(&mut Transform, &Speed, &mut TargetPos), With<Unit>>,
    time: Res<Time>,
) {
    for (mut trans, speed, mut target_pos) in unit_q.iter_mut() {
        let Some(new_pos) = target_pos.0 else {
            return;
        };

        let distance = new_pos - trans.translation;

        if distance.length_squared() <= (speed.0 * time.delta_seconds()).powi(2) {
            trans.translation = new_pos;
            target_pos.0 = None;
            println!("Unit Stopping");
        } else {
            trans.translation += distance.normalize() * speed.0 * time.delta_seconds();
        }
    }
}

// referenced https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn set_ground_coords(
    mut ground_coords: ResMut<GroundCoords>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
    map_base_q: Query<&GlobalTransform, With<MapBase>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let map_base_trans = map_base_q.single();
    let window = window_q.single();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let plane_origin = map_base_trans.translation();
    let plane = Plane3d::new(map_base_trans.up());
    let Some(ray) = cam.viewport_to_world(cam_trans, cursor_pos) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        return;
    };
    let global_cursor = ray.get_point(distance);

    ground_coords.global = global_cursor;

    let inverse_transform_matrix = map_base_trans.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);
    ground_coords.local = local_cursor.xz();
}

pub fn deselect(
    mut cmds: Commands,
    mut select_q: Query<(Entity, &mut Selected)>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Right) {
        for (entity, mut select) in select_q.iter_mut() {
            if select.0 {
                select.0 = false;
                println!("Unit deselected");
                cmds.entity(entity).insert(ColliderDebugColor(Color::NONE));
            }
        }
    }
}

pub fn select(
    mut cmds: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    cam: Query<(&Camera, &GlobalTransform)>,
    mut select_q: Query<&mut Selected>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = window.single();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let (cam, cam_trans) = cam.single();

    let Some(ray) = cam.viewport_to_world(cam_trans, cursor_pos) else {
        return;
    };

    let hit = rapier_context.cast_ray(
        ray.origin,
        ray.direction.into(),
        f32::MAX,
        true,
        QueryFilter::only_dynamic(),
    );

    if let Some((entity, _toi)) = hit {
        if let Ok(mut selected) = select_q.get_mut(entity) {
            selected.0 = !selected.0;

            let mut color = Color::NONE;
            if selected.0 {
                color = Color::GREEN;
            } else {
                println!("Unit Deselected");
            }

            cmds.entity(entity).insert(ColliderDebugColor(color));
        }
    }
}
