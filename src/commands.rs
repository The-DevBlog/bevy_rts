use std::f32::EPSILON;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{
    dynamics::ExternalImpulse, pipeline::QueryFilter, plugin::RapierContext,
    render::ColliderDebugColor,
};
use bevy_rts_camera::RtsCamera;

use crate::{
    resources::{BoxSelect, MouseCoords},
    MapBase, Selected, Speed, TargetPos, Unit,
};

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MyRoundGizmos>().add_systems(
            Update,
            (
                set_unit_destination,
                set_mouse_coords,
                move_unit,
                single_select,
                drag_select,
                deselect,
            ),
        );
    }
}

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

fn set_unit_destination(
    mouse_coords: ResMut<MouseCoords>,
    mut unit_q: Query<(&mut TargetPos, &Transform), With<Selected>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Left) {
        for (mut unit_target_pos, trans) in unit_q.iter_mut() {
            let mut destination = mouse_coords.global;
            destination.y += trans.scale.y / 2.0; // calculate for entity height
            unit_target_pos.0 = Some(destination);
            println!(
                "global mouse: {:?}, unit: {:?}",
                mouse_coords.local,
                unit_target_pos.0.unwrap()
            );
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
fn set_mouse_coords(
    mut mouse_coords: ResMut<MouseCoords>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
    map_base_q: Query<&GlobalTransform, With<MapBase>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let map_base_trans = map_base_q.single();
    let window = window_q.single();
    let Some(local_cursor) = window.cursor_position() else {
        return;
    };

    let plane_origin = map_base_trans.translation();
    let plane = Plane3d::new(map_base_trans.up());
    let Some(ray) = cam.viewport_to_world(cam_trans, local_cursor) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        return;
    };
    let global_cursor = ray.get_point(distance);

    mouse_coords.global = global_cursor;
    mouse_coords.local = local_cursor;
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
    mut gizmos: Gizmos,
    unit_q: Query<(Entity, &Transform), With<Unit>>,
    mouse_coords: Res<MouseCoords>,
    mut box_select: ResMut<BoxSelect>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Left) {
        box_select.global_start = mouse_coords.global;
        box_select.local_start = mouse_coords.local;
    }

    if input.pressed(MouseButton::Left) {
        box_select.local_end = mouse_coords.local;
        box_select.global_end = mouse_coords.global;

        let start = box_select.global_start;
        let end = box_select.global_end;

        // draw rectangle
        gizmos.line(start, Vec3::new(end.x, 0.0, start.z), Color::GRAY);
        gizmos.line(start, Vec3::new(start.x, 0.0, end.z), Color::GRAY);
        gizmos.line(Vec3::new(start.x, 0.0, end.z), end, Color::GRAY);
        gizmos.line(Vec3::new(end.x, 0.0, start.z), end, Color::GRAY);

        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        let min_z = start.z.min(end.z);
        let max_z = start.z.max(end.z);

        for (unit_ent, unit_trans) in unit_q.iter() {
            let unit_pos = unit_trans.translation;
            if unit_pos.x >= min_x
                && unit_pos.x <= max_x
                && unit_pos.z >= min_z
                && unit_pos.z <= max_z
            {
                cmds.entity(unit_ent)
                    .insert((ColliderDebugColor(Color::GREEN), Selected));
            }
        }
    }
}

pub fn single_select(
    mut cmds: Commands,
    rapier_context: Res<RapierContext>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    select_q: Query<(Entity, &Selected)>,
    input: Res<ButtonInput<MouseButton>>,
    mouse_coords: Res<MouseCoords>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let (cam, cam_trans) = cam_q.single();

    let Some(ray) = cam.viewport_to_world(cam_trans, mouse_coords.local) else {
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
