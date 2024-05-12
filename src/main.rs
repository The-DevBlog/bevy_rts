mod camera;
mod components;
mod map;
mod resources;
mod units;
mod utils;

use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use camera::CameraPlugin;
use components::*;
use map::MapPlugin;
use resources::ResourcesPlugin;
use units::UnitsPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const MAP_SIZE: f32 = 400.0;
const UNITS: i32 = 100;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            MapPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            ResourcesPlugin,
            UtilsPlugin,
            UnitsPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
