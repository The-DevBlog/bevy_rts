use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rts_camera::RtsCamera;

use crate::{
    resources::{BoxCoords, DragSelect, MouseCoords},
    MapBase,
};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_box_coords, set_mouse_coords, drag_select));
    }
}

fn drag_select(box_coords: Res<BoxCoords>, mut drag_select: ResMut<DragSelect>) {
    let drag_threshold = 2.5;
    let width_z = (box_coords.global_start.z - box_coords.global_end.z).abs();
    let width_x = (box_coords.global_start.x - box_coords.global_end.x).abs();

    if width_z > drag_threshold || width_x > drag_threshold {
        drag_select.0 = true;
    }
}

fn set_box_coords(
    mut box_coords: ResMut<BoxCoords>,
    mut drag_select: ResMut<DragSelect>,
    input: Res<ButtonInput<MouseButton>>,
    mouse_coords: Res<MouseCoords>,
) {
    if input.just_pressed(MouseButton::Left) {
        box_coords.global_start = mouse_coords.global;
        box_coords.local_start = mouse_coords.local;
    }

    if input.pressed(MouseButton::Left) {
        box_coords.local_end = mouse_coords.local;
        box_coords.global_end = mouse_coords.global;
    }

    if input.just_released(MouseButton::Left) {
        box_coords.empty_global();
        drag_select.0 = false;
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
