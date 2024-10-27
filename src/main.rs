use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    // render::RapierDebugRenderPlugin,
};

use bevy::prelude::*;

mod camera;
mod components;
mod events;
mod map;
mod mouse;
mod resources;
mod tank;
mod utils;

use camera::CameraPlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use tank::TankPlugin;
use utils::UtilsPlugin;

const TANK_COUNT: usize = 25;
const MAP_SIZE: f32 = 800.0;
const SPEED_QUANTIFIER: f32 = 1000.0;
const TANK_SPEED: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
            ResourcesPlugin,
            BillboardPlugin,
            CameraPlugin,
            MapPlugin,
            MousePlugin,
            UtilsPlugin,
            TankPlugin,
        ))
        .run();
}
