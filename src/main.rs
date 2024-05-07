mod camera;
mod commands;
mod map;
mod units;

use camera::CameraPlugin;
use commands::CommandsPlugin;
use map::MapPlugin;
use units::UnitsPlugin;

use bevy::prelude::*;

const MAP_SIZE: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            UnitsPlugin,
            MapPlugin,
            CommandsPlugin,
        ))
        .run();
}
