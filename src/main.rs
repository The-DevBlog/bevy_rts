use bevy::{color::palettes::css::*, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_rts_pathfinding::BevyRtsPathFindingPlugin;

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

const COLOR_SELECT_BOX: Color = Color::srgba(0.68, 0.68, 0.68, 0.25);
const COLOR_SELECT_BOX_BORDER: Srgba = DARK_GRAY;
const COLOR_PATH_FINDING: Srgba = YELLOW;
const COLOR_PATH: Srgba = LIGHT_STEEL_BLUE;
const COLOR_OCCUPIED_CELL: Srgba = RED;
const COLOR_GRID: Srgba = GRAY;
const CURSOR_SIZE: f32 = 25.0;
const MAP_SIZE: f32 = 800.0;
const MAP_GRID_SIZE: u32 = 60;
const MAP_CELL_SIZE: f32 = MAP_SIZE / MAP_GRID_SIZE as f32;
const SPEED_QUANTIFIER: f32 = 1000.0;
const TANK_COUNT: usize = 1;
const TANK_SPEED: f32 = 75.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
            BevyRtsPathFindingPlugin,
            ResourcesPlugin,
            BillboardPlugin,
            CameraPlugin,
            MapPlugin,
            MousePlugin,
            TankPlugin,
        ))
        .run();
}
