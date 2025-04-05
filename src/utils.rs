use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

pub fn cast_ray(
    rapier: &RapierContext,
    cam: &Camera,
    cam_trans: &GlobalTransform,
    viewport: Vec2,
) -> Option<(Entity, f32)> {
    let Ok(ray) = cam.viewport_to_world(cam_trans, viewport) else {
        return None;
    };

    let filter = QueryFilter::default().exclude_sensors();
    let hit = rapier.cast_ray(ray.origin, ray.direction.into(), f32::MAX, true, filter);

    return hit;
}

// TODO: throws excception if ray is off plane I think
pub fn get_world_coords(
    map_base_trans: &GlobalTransform,
    cam_trans: &GlobalTransform,
    cam: &Camera,
    viewport_pos: Vec2,
) -> Option<Vec3> {
    let plane_origin = map_base_trans.translation();
    let plane = InfinitePlane3d::new(map_base_trans.up());
    let ray = cam.viewport_to_world(cam_trans, viewport_pos).unwrap();
    let distance = ray.intersect_plane(plane_origin, plane);

    if let Some(distance) = distance {
        return Some(ray.get_point(distance));
    }

    None
}

pub fn billboard_sync(
    cam: &Camera,
    cam_trans: &GlobalTransform,
    window: &Window,
    transform: &Transform,
    obj_size: Vec2,
    style: &mut Node,
    min_size: f32,
) {
    // Get the unit's center in screen space.
    let center_screen = match cam.world_to_viewport(cam_trans, transform.translation) {
        Ok(pos) => pos,
        Err(_) => return,
    };

    // Compute the distance from the camera to the unit.
    let distance = cam_trans.translation().distance(transform.translation);

    // Use the formula:
    // scale = (window_height/2) / (distance * tan(fov_y/2))
    let window_height = window.physical_height() as f32;
    let scale = (window_height / 2.0) / (distance * (FRAC_PI_2 / 2.0).tan());

    // Compute the screen-space width and height of the unit.
    // let min_size = 20.0; // pixels, adjust as needed
    let screen_width = (obj_size.x * scale).max(min_size);
    let screen_height = (obj_size.y * scale).max(min_size);

    style.left = Val::Px(center_screen.x - screen_width / 2.0);
    style.top = Val::Px(center_screen.y - screen_height / 2.0);
    style.width = Val::Px(screen_width);
    style.height = Val::Px(screen_height);
}
