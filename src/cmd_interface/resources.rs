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
}
