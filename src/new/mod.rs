mod camera;
mod components;
mod friendly;
mod map;
mod mouse;
mod resources;
mod tank;
mod utils;

use camera::CameraPlugin;
use friendly::FriendlyPlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use tank::TankPlugin;
use utils::UtilsPlugin;

use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

const TANK_COUNT: usize = 100;
const MAP_SIZE: f32 = 800.0;
const SPEED_QUANTIFIER: f32 = 1000.0;
const TANK_SPEED: f32 = 50.0;

pub struct NewPlugin;

impl Plugin for NewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UtilsPlugin, FriendlyPlugin, TankPlugin))
            .add_plugins((
                CameraPlugin,
                MapPlugin,
                BillboardPlugin,
                ResourcesPlugin,
                MousePlugin,
            ));
    }
}
