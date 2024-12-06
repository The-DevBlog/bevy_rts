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

    let hit = rapier.cast_ray(
        ray.origin,
        ray.direction.into(),
        f32::MAX,
        true,
        QueryFilter::only_dynamic(),
    );

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
