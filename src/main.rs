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
const CURSOR_SIZE: f32 = 25.0;
const MAP_WIDTH: f32 = CELL_SIZE * MAP_GRID_COLUMNS as f32;
const MAP_DEPTH: f32 = CELL_SIZE * MAP_GRID_ROWS as f32;
const CELL_SIZE: f32 = 10.0;
const MAP_GRID_COLUMNS: usize = 40;
const MAP_GRID_ROWS: usize = 40;
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
            bevy_rts_pathfinding::BevyRtsPathFindingPlugin,
            bevy_rts_pathfinding::debug::DebugPlugin,
            ResourcesPlugin,
            BillboardPlugin,
            CameraPlugin,
            MapPlugin,
            MousePlugin,
            TankPlugin,
        ))
        .run();
}
