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
mod shaders;
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
use shaders::ShadersPlugin;
use structures::StructuresPlugin;
use tank::TankPlugin;
use units::UnitsPlugin;

// const COLOR_GROUND: Color = Color::srgb(0.44, 0.75, 0.44);
const COLOR_GROUND: Color = Color::srgb(0.42, 0.61, 0.38);
const COLOR_SELECT_BOX: Color = Color::srgba(0.0, 0.45, 0.73, 0.45); // TODO: Keeps getting modified somehow?
const COLOR_SELECT_BOX_BORDER: Color = Color::srgba(0.22, 0.22, 0.22, 1.0);
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
const SPEED_ARTILLERY: f32 = SPEED_QUANTIFIER * 40.0;

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let shorts_flag = args.contains(&String::from("-shorts"));

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins((
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

    app.add_plugins((AudioPlugin, SpatialAudioPlugin, ShadersPlugin));

    // if !shorts_flag {
    //     app.add_plugins(WorldInspectorPlugin::new());
    // }

    app.run();
}
