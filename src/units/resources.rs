use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnlockedUnits>();
    }
}

#[derive(Resource, Default, Debug)]
pub struct UnlockedUnits {
    pub rifleman: bool,  // barracks built
    pub tank_gen1: bool, // vehicle depot built
    pub tank_gen2: bool, // vehicle depot build, (eventually research as well)
}
