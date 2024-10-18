mod ai_enemy;
mod animation_controller;
mod camera;
mod components;
mod events;
mod friendly;
mod hud;
mod map;
mod mouse;
mod resources;
mod soldiers;
mod tanks;
mod tanks_2;
mod utils;

use ai_enemy::AiEnemyPlugin;
use animation_controller::AnimationControllerPlugin;
use camera::CameraPlugin;
use events::*;
use friendly::FriendlyPlugin;
use hud::HudPlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use soldiers::SoldiersPlugin;
use tanks::TanksPlugin;
use tanks_2::Tanks2Plugin;
use utils::UtilsPlugin;

use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

const TANK_COUNT: usize = 100;
const STARTING_FUNDS: i32 = 2500;
const MAP_SIZE: f32 = 800.0;
const SPEED_QUANTIFIER: f32 = 1000.0;
const SOLDIER_DMG: f32 = 20.0;
const SOLDIER_HEALTH: f32 = 250.0;
const SOLDIER_RANGE: f32 = 50.0;
const SOLDIER_SPEED: f32 = 6.5;
const SOLDIER_FIRE_RATE: f32 = 1.5;
const SOLDIER_COST: i32 = 50;
const SOLDIER_REWARD: i32 = 25;
const TANK_DMG: f32 = 80.0;
const TANK_SPEED: f32 = 50.0;
const TANK_FIRE_RATE: f32 = 1.5;
const TANK_COST: i32 = 250;
const TANK_RANGE: f32 = 125.0;
const TANK_HEALTH: f32 = 1000.0;
const TANK_REWARD: i32 = 100;
const TANK_FACTORY_HEALTH: f32 = 3000.0;
const BARRACKS_HEALTH: f32 = 2000.0;

pub struct NewPlugin;

impl Plugin for NewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UtilsPlugin,
            AnimationControllerPlugin,
            FriendlyPlugin,
            AiEnemyPlugin,
            Tanks2Plugin,
        ))
        .add_plugins((
            CameraPlugin,
            MapPlugin,
            HudPlugin,
            BillboardPlugin,
            ResourcesPlugin,
            TanksPlugin,
            SoldiersPlugin,
            MousePlugin,
            EventsPlugin,
        ));
    }
}
