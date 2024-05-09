mod camera;
mod commands;
mod components;
mod map;
mod resources;
mod units;

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

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const MAP_SIZE: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            UnitsPlugin,
            MapPlugin,
            CommandsPlugin,
            ResourcesPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .run();
}
