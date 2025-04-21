use bevy::{color::palettes::css::*, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::{AudioPlugin, SpatialAudioPlugin};
use bevy_mod_outline::OutlinePlugin;
use bevy_rapier3d::prelude::*;
use bevy_rts_pathfinding;

mod asset_manager;
mod bank;
mod camera;
mod cmd_interface;
mod events;
mod map;
mod mouse;
mod resources;
mod structures;
mod tank;
mod units;
mod utils;

use asset_manager::AssetManagerPlugin;
use bank::BankPlugin;
use camera::CameraPlugin;
use cmd_interface::CmdInterfacePlugin;
use map::MapPlugin;
use mouse::MousePlugin;
use resources::ResourcesPlugin;
use structures::StructuresPlugin;
use tank::TankPlugin;
use units::UnitsPlugin;

const TINT_STRENGTH: f32 = 0.4;
const TINT_CLR: Srgba = YELLOW;
const COLOR_SELECT_BOX: Color = Color::srgba(0.68, 0.68, 0.68, 0.25);
const COLOR_SELECT_BOX_BORDER: Srgba = DARK_GRAY;
const CELL_SIZE: f32 = 10.0;
const MAP_WIDTH: f32 = CELL_SIZE * MAP_GRID_COLUMNS as f32;
const MAP_DEPTH: f32 = CELL_SIZE * MAP_GRID_ROWS as f32;
const MAP_GRID_COLUMNS: i32 = 120;
const MAP_GRID_ROWS: i32 = 120;
const TANK_COUNT: usize = 50;
const SPEED_QUANTIFIER: f32 = 10.0;
const SPEED_RIFELMAN: f32 = SPEED_QUANTIFIER * 10.0;
const SPEED_TANK_GEN_1: f32 = SPEED_QUANTIFIER * 50.0;
const SPEED_TANK_GEN_2: f32 = SPEED_QUANTIFIER * 50.0;

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let shorts_flag = args.contains(&String::from("-shorts"));

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        CmdInterfacePlugin,
        AssetManagerPlugin,
        BankPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        // RapierDebugRenderPlugin::default(),
        WorldInspectorPlugin::new(),
        OutlinePlugin,
        bevy_rts_pathfinding::BevyRtsPathFindingPlugin,
        ResourcesPlugin,
        StructuresPlugin,
        CameraPlugin,
        UnitsPlugin,
        MapPlugin,
        MousePlugin,
        TankPlugin,
    ));

    app.add_plugins((AudioPlugin, SpatialAudioPlugin));

    // if !shorts_flag {
    //     app.add_plugins(WorldInspectorPlugin::new());
    // }

    app.run();
}
