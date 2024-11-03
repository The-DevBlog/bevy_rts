use bevy::prelude::*;
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

use crate::{MAP_CELL_SIZE, MAP_SIZE};

pub fn cast_ray(
    rapier: Res<RapierContext>,
    cam: &Camera,
    cam_trans: &GlobalTransform,
    viewport: Vec2,
) -> Option<(Entity, f32)> {
    let Some(ray) = cam.viewport_to_world(cam_trans, viewport) else {
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

pub fn get_world_coords(
    map_base_trans: &GlobalTransform,
    cam_trans: &GlobalTransform,
    cam: &Camera,
    viewport_pos: Vec2,
) -> Vec3 {
    let plane_origin = map_base_trans.translation();
    let plane = InfinitePlane3d::new(map_base_trans.up());
    let ray = cam.viewport_to_world(cam_trans, viewport_pos).unwrap();
    let distance = ray.intersect_plane(plane_origin, plane).unwrap();
    return ray.get_point(distance);
}

pub fn get_unit_cell_row_and_column(transform: &Transform) -> (u32, u32) {
    // Get the unit's current cell
    let unit_pos = transform.translation;
    let grid_origin = -MAP_SIZE / 2.0;
    let adjusted_x = unit_pos.x - grid_origin;
    let adjusted_z = unit_pos.z - grid_origin;

    let column = (adjusted_x / MAP_CELL_SIZE).floor() as u32;
    let row = (adjusted_z / MAP_CELL_SIZE).floor() as u32;

    (row, column)
}
