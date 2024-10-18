mod camera;
mod components;
mod map;
mod mouse;
mod resources;
mod tanks;
mod utils;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use camera::CameraPlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use tanks::TanksPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;

const MAP_SIZE: f32 = 400.0;
const TANK_SPEED: f32 = 50.0;
const SPEED_QUANTIFIER: f32 = 1000.0;
const TANK_COUNT: usize = 100;

pub struct OriginalPlugin;

impl Plugin for OriginalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ResourcesPlugin,
            CameraPlugin,
            MousePlugin,
            MapPlugin,
            TanksPlugin,
            UtilsPlugin,
        ));
    }
}
