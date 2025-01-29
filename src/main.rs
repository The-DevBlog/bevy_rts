use bevy::{color::palettes::css::*, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
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
const CELL_SIZE: f32 = 7.5;
const MAP_WIDTH: f32 = CELL_SIZE * MAP_GRID_COLUMNS as f32;
const MAP_DEPTH: f32 = CELL_SIZE * MAP_GRID_ROWS as f32;
const MAP_GRID_COLUMNS: i32 = 120;
const MAP_GRID_ROWS: i32 = 120;
const SPEED_QUANTIFIER: f32 = 10.0;
const TANK_COUNT: usize = 100;
const TANK_SPEED: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
            bevy_rts_pathfinding::BevyRtsPathFindingPlugin,
            ResourcesPlugin,
            CameraPlugin,
            MapPlugin,
            MousePlugin,
            TankPlugin,
        ))
        .run();
}
