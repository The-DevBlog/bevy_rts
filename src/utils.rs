use bevy::prelude::*;
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

pub fn helper(
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
