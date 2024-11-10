use bevy::{color::palettes::css::*, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_rts_pathfinding;
mod camera;
mod components;
mod events;
mod map;
mod mouse;
mod pathfinding;
mod resources;
mod tank;
mod utils;

use camera::CameraPlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use pathfinding::PathFindingPlugin;
use resources::ResourcesPlugin;
use tank::TankPlugin;

const COLOR_SELECT_BOX: Color = Color::srgba(0.68, 0.68, 0.68, 0.25);
const COLOR_SELECT_BOX_BORDER: Srgba = DARK_GRAY;
// const COLOR_PATH_FINDING: Srgba = YELLOW;
// const COLOR_PATH: Srgba = LIGHT_STEEL_BLUE;
// const COLOR_OCCUPIED_CELL: Srgba = RED;
// const COLOR_GRID: Srgba = GRAY;
const CURSOR_SIZE: f32 = 25.0;
const MAP_WIDTH: f32 = CELL_SIZE * MAP_GRID_COLUMNS as f32;
const MAP_DEPTH: f32 = CELL_SIZE * MAP_GRID_ROWS as f32;
const CELL_SIZE: f32 = 10.0;
const MAP_GRID_COLUMNS: usize = 40;
const MAP_GRID_ROWS: usize = 40;
// const MAP_CELL_WIDTH: f32 = MAP_WIDTH / MAP_GRID_COLUMNS as f32;
// const MAP_CELL_HEIGHT: f32 = MAP_HEIGHT / MAP_GRID_ROWS as f32;
const SPEED_QUANTIFIER: f32 = 1000.0;
const TANK_COUNT: usize = 5;
const TANK_SPEED: f32 = 75.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
            PathFindingPlugin,
            bevy_rts_pathfinding::BevyRtsPathFindingPlugin,
            bevy_rts_pathfinding::debug_plugin::DebugPlugin,
            ResourcesPlugin,
            BillboardPlugin,
            CameraPlugin,
            MapPlugin,
            MousePlugin,
            TankPlugin,
        ))
        .run();
}
