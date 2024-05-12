mod camera;
mod commands;
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
use commands::CommandsPlugin;
use components::*;
use map::MapPlugin;
use resources::ResourcesPlugin;
use units::UnitsPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const MAP_SIZE: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            CommandsPlugin,
            MapPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            ResourcesPlugin,
            UnitsPlugin,
            UtilsPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
