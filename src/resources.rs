use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GroundCoords>();
    }
}

#[derive(Resource, Default, Debug)]
pub struct GroundCoords {
    pub global: Vec3,
    pub local: Vec2,
}
