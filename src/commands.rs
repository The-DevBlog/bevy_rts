use std::f32::EPSILON;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{
    dynamics::ExternalImpulse, pipeline::QueryFilter, plugin::RapierContext,
    render::ColliderDebugColor,
};
use bevy_rts_camera::RtsCamera;

use crate::{
    resources::{BoxSelect, GroundCoords},
    MapBase, Selected, Speed, TargetPos, Unit,
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
                drag_select,
                deselect,
            ),
        );
    }
}

fn set_unit_destination(
    ground_coords: ResMut<GroundCoords>,
    mut unit_q: Query<(&mut TargetPos, &Transform), With<Selected>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Left) {
        for (mut unit_target_pos, trans) in unit_q.iter_mut() {
            let mut destination = ground_coords.global;
            destination.y += trans.scale.y / 2.0; // calculate for entity height
            unit_target_pos.0 = Some(destination);
            println!("Unit Moving");
        }
    }
}

fn move_unit(
    mut unit_q: Query<(&mut Transform, &mut ExternalImpulse, &Speed, &mut TargetPos), With<Unit>>,
    time: Res<Time>,
) {
    for (trans, mut ext_impulse, speed, mut target_pos) in unit_q.iter_mut() {
        if let Some(new_pos) = target_pos.0 {
            let distance = new_pos - trans.translation;
            if distance.length_squared() <= (speed.0 * time.delta_seconds()).powi(2) + EPSILON {
                target_pos.0 = None;
                println!("Unit Stopping");
            } else {
                ext_impulse.impulse += distance.normalize() * speed.0 * time.delta_seconds();
            }
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
    mut select_q: Query<Entity, With<Selected>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Right) {
        for entity in select_q.iter_mut() {
            println!("Unit deselected");
            cmds.entity(entity).insert(ColliderDebugColor(Color::NONE));
            cmds.entity(entity).remove::<Selected>();
        }
    }
}

pub fn drag_select(
    mut cmds: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut box_select: ResMut<BoxSelect>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    map_base_q: Query<&GlobalTransform, With<MapBase>>,
    input: Res<ButtonInput<MouseButton>>,
    unit_q: Query<(Entity, &Transform), With<Unit>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let map_base_trans = map_base_q.single();
    let window = window_q.single();

    if input.just_pressed(MouseButton::Left) {
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
        box_select.start = global_cursor;
    }

    if input.just_released(MouseButton::Left) {
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
        box_select.end = global_cursor;

        let min_x = box_select.start.x.min(box_select.end.x);
        let max_x = box_select.start.x.max(box_select.end.x);
        let min_z = box_select.start.z.min(box_select.end.z);
        let max_z = box_select.start.z.max(box_select.end.z);

        for (unit_ent, unit_trans) in unit_q.iter() {
            let unit_pos = unit_trans.translation;
            if unit_pos.x >= min_x
                && unit_pos.x <= max_x
                && unit_pos.z >= min_z
                && unit_pos.z <= max_z
            {
                cmds.entity(unit_ent)
                    .insert((ColliderDebugColor(Color::GREEN), Selected));
            } else {
                println!("OUT");
            }
        }
    }
}

pub fn select(
    mut cmds: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    select_q: Query<(Entity, &Selected)>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = window_q.single();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let (cam, cam_trans) = cam_q.single();

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
        // if unit is already selected, remove Selected component
        if let Ok((_, _)) = select_q.get(entity) {
            cmds.entity(entity)
                .remove::<Selected>()
                .insert(ColliderDebugColor(Color::NONE));
        // else add the Selected component
        } else {
            cmds.entity(entity)
                .insert((ColliderDebugColor(Color::GREEN), Selected));
        }
    }
}
