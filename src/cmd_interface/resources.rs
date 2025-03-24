use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InfoContainerData>();
    }
}

#[derive(Resource, Default)]
pub struct InfoContainerData {
    pub active: bool,
    pub name: String,
    pub cost: i32,
    pub build_time: i32,
    pub hp: Option<i32>,
    pub dmg: Option<i32>,
    pub speed: Option<f32>,
}
