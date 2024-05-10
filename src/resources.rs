use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GroundCoords>()
            .init_resource::<BoxSelect>();
    }
}

#[derive(Resource, Default, Debug)]
pub struct GroundCoords {
    pub global: Vec3,
    pub local: Vec2,
}

#[derive(Resource, Default, Debug)]
pub struct BoxSelect {
    // pub global_start: Vec3,
    // pub global_end: Vec3,
    // pub local_start: Vec2,
    // pub local_end: Vec2,
    pub start: Vec3,
    pub end: Vec3,
}
