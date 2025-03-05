use bevy::prelude::*;

use crate::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildStructurePlaceholder>();
    }
}

#[derive(Resource)]
pub struct BuildStructurePlaceholder(pub Option<Entity>);
