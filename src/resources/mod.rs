use bevy::prelude::*;

pub mod units;

use units::UnitsPlugin;

use crate::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        let args: Vec<String> = std::env::args().collect();
        let debug_flag = args.contains(&String::from("-debug"));

        app.add_plugins(UnitsPlugin)
            .init_resource::<MouseCoords>()
            .init_resource::<SelectBox>()
            .init_resource::<GameCommands>()
            .init_resource::<MyAssets>()
            .init_resource::<CursorState>()
            .insert_resource(DbgOptions {
                print_statements: debug_flag,
            });
    }
}

#[derive(Reflect, Resource, Clone, Copy)]
#[reflect(Resource)]
pub struct DbgOptions {
    pub print_statements: bool,
}

impl DbgOptions {
    pub fn print(&self, msg: &str) {
        if self.print_statements {
            println!("{}", msg);
        }
    }
}

#[derive(Resource, Default)]
pub struct MyAssets {
    // pub models: Models,
    pub imgs: Images,
    pub textures: Textures,
}

#[derive(Default)]
pub struct Textures {
    pub grass_clr: Handle<Image>,
    pub grass_normal: Handle<Image>,
    pub grass_roughness: Handle<Image>,
    pub grass_occlusion: Handle<Image>,
}

#[derive(Default)]
pub struct Images {
    pub select_border: Handle<Image>,
    pub cursor_relocate: Handle<Image>,
    pub cursor_select: Handle<Image>,
    pub cursor_standard: Handle<Image>,
    pub cmd_intrfce_structures: Handle<Image>,
    pub cmd_intrfce_units: Handle<Image>,
    pub cmd_intrfce_background: Handle<Image>,
    pub info_ctr: Handle<Image>,
    pub info_ctr_dmg: Handle<Image>,
    pub info_ctr_speed: Handle<Image>,
    pub info_ctr_build_time: Handle<Image>,
    pub info_ctr_hp: Handle<Image>,
    pub structure_barracks: Handle<Image>,
    pub structure_cannon: Handle<Image>,
    pub structure_vehicle_depot: Handle<Image>,
    pub structure_research_center: Handle<Image>,
    pub structure_satellite_dish: Handle<Image>,
    pub unit_tank_gen1: Handle<Image>,
    pub unit_tank_gen2: Handle<Image>,
    pub unit_rifleman: Handle<Image>,
}

// #[derive(Default)]
// pub struct Models {
//     pub barracks: Handle<Scene>,
//     pub tank_gen1: Handle<Scene>,
//     pub tank_gen2: Handle<Scene>,
//     pub rifleman: Handle<Scene>,
//     pub cannon: Handle<Scene>,
//     pub vehicle_depot: Handle<Scene>,
//     pub research_center: Handle<Scene>,
//     pub satellite_dish: Handle<Scene>,
//     pub placeholders: Placeholders,
// }

// #[derive(Default)]
// pub struct Placeholders {
//     pub barracks_valid: Handle<Scene>,
//     pub barracks_invalid: Handle<Scene>,
//     pub cannon_valid: Handle<Scene>,
//     pub cannon_invalid: Handle<Scene>,
//     pub vehicle_depot_valid: Handle<Scene>,
//     pub vehicle_depot_invalid: Handle<Scene>,
//     pub research_center_valid: Handle<Scene>,
//     pub research_center_invalid: Handle<Scene>,
//     pub satellite_dish_valid: Handle<Scene>,
//     pub satellite_dish_invalid: Handle<Scene>,
// }

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

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub enum CursorState {
    Relocate,
    Select,
    Build,
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
    pub is_any_unit_selected: bool,
    pub hvr_cmd_interface: bool,
}
