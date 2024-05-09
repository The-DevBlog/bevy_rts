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
    pub start: Vec3,
    pub end: Vec3,
}
