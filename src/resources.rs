use bevy::prelude::*;

use crate::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseCoords>()
            .init_resource::<SelectBox>()
            .init_resource::<GameCommands>()
            .init_resource::<MyAssets>()
            .init_resource::<CursorState>()
            .add_systems(PreStartup, add_assets);
    }
}

#[derive(Resource, Default)]
pub struct MyAssets {
    pub select_border: Handle<Image>,
    pub cursor_relocate: Handle<Image>,
    pub cursor_select: Handle<Image>,
    pub cursor_standard: Handle<Image>,
    pub cmd_cntr_structures: Handle<Image>,
    pub cmd_cntr_units: Handle<Image>,
}

#[derive(Resource, Default, Debug)]
pub struct MouseCoords {
    pub world: Vec3,
    pub viewport: Vec2,
}

impl MouseCoords {
    pub fn in_bounds(&self) -> bool {
        if self.world.x.abs() > MAP_WIDTH / 2.0 || self.world.z.abs() > MAP_DEPTH / 2.0 {
            return false;
        }

        return true;
    }
}

#[derive(Resource, Default, Debug)]
pub struct SelectBox {
    pub viewport: Viewport,
    pub world: World,
}

impl SelectBox {
    pub fn empty_coords(&mut self) {
        self.viewport.start_1 = Vec2::ZERO;
        self.viewport.start_2 = Vec2::ZERO;
        self.viewport.end_1 = Vec2::ZERO;
        self.viewport.end_2 = Vec2::ZERO;
        self.world.start_1 = Vec3::ZERO;
        self.world.start_2 = Vec3::ZERO;
        self.world.end_1 = Vec3::ZERO;
        self.world.end_2 = Vec3::ZERO;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Viewport {
    pub start_1: Vec2,
    pub start_2: Vec2,
    pub end_1: Vec2,
    pub end_2: Vec2,
}

impl Viewport {
    pub fn initialize_coords(&mut self, coords: Vec2) {
        self.start_1 = coords;
        self.start_2 = coords;
        self.end_1 = coords;
        self.end_2 = coords;
    }
}

#[derive(Default, Debug, Clone)]
pub struct World {
    pub start_1: Vec3,
    pub start_2: Vec3,
    pub end_1: Vec3,
    pub end_2: Vec3,
}

impl World {
    pub fn initialize_coords(&mut self, coords: Vec3) {
        self.start_1 = coords;
        self.start_2 = coords;
        self.end_1 = coords;
        self.end_2 = coords;
    }
}

#[derive(Resource, Debug, PartialEq)]
pub enum CursorState {
    Relocate,
    Select,
    Standard,
}

impl Default for CursorState {
    fn default() -> Self {
        CursorState::Standard
    }
}

#[derive(Resource, Default, Debug)]
pub struct GameCommands {
    pub drag_select: bool,
    pub is_any_selected: bool,
}

fn add_assets(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
    my_assets.select_border = assets.load("imgs/select_border.png");
    my_assets.cursor_relocate = assets.load("imgs/cursor/relocate.png");
    my_assets.cursor_select = assets.load("imgs/cursor/select.png");
    my_assets.cursor_standard = assets.load("imgs/cursor/standard.png");
    my_assets.cmd_cntr_structures = assets.load("imgs/cmd_cntr_structures.png");
    my_assets.cmd_cntr_units = assets.load("imgs/cmd_cntr_units.png");
}
