use bevy::{
    image::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
    prelude::*,
};

use crate::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        let args: Vec<String> = std::env::args().collect();
        let debug_flag = args.contains(&String::from("-debug"));

        app.init_resource::<MouseCoords>()
            .init_resource::<SelectBox>()
            .init_resource::<GameCommands>()
            .init_resource::<MyAssets>()
            .init_resource::<CursorState>()
            .init_resource::<AvailableUnits>()
            .init_resource::<StructuresBuilt>()
            .insert_resource(DbgOptions {
                print_statements: debug_flag,
            })
            .add_systems(PreStartup, add_assets);
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
    pub models: Models,
    pub imgs: Images,
    pub textures: Textures,
    pub audio: Audio,
}

#[derive(Default)]
pub struct Audio {
    pub place_structure: Handle<AudioSource>,
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
    pub structure_barracks: Handle<Image>,
    pub structure_cannon: Handle<Image>,
    pub structure_vehicle_depot: Handle<Image>,
    pub structure_research_center: Handle<Image>,
    pub structure_satellite_dish: Handle<Image>,
    pub unit_tank_gen1: Handle<Image>,
    pub unit_tank_gen2: Handle<Image>,
}

#[derive(Default)]
pub struct Models {
    pub barracks: Handle<Scene>,
    pub tank_gen1: Handle<Scene>,
    pub tank_gen2: Handle<Scene>,
    pub cannon: Handle<Scene>,
    pub vehicle_depot: Handle<Scene>,
    pub research_center: Handle<Scene>,
    pub satellite_dish: Handle<Scene>,
    pub placeholders: Placeholders,
}

#[derive(Default)]
pub struct Placeholders {
    pub barracks_valid: Handle<Scene>,
    pub barracks_invalid: Handle<Scene>,
    pub cannon_valid: Handle<Scene>,
    pub cannon_invalid: Handle<Scene>,
    pub vehicle_depot_valid: Handle<Scene>,
    pub vehicle_depot_invalid: Handle<Scene>,
    pub research_center_valid: Handle<Scene>,
    pub research_center_invalid: Handle<Scene>,
    pub satellite_dish_valid: Handle<Scene>,
    pub satellite_dish_invalid: Handle<Scene>,
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
    pub is_any_selected: bool,
    pub hvr_cmd_interface: bool,
}

#[derive(Resource, Default)]
pub struct StructuresBuilt {
    pub barracks: bool,
    pub cannon: bool,
    pub vehicle_depot: bool,
    pub research_center: bool,
    pub satellite_dish: bool,
}

#[derive(Resource, Default)]
pub struct AvailableUnits {
    pub tank_gen1: bool,
    pub tank_gen2: bool,
}

fn add_assets(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
    // textures
    my_assets.textures.grass_clr =
        assets.load_with_settings("textures/grass/color.png", |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        });

    my_assets.textures.grass_normal =
        assets.load_with_settings("textures/grass/normal_gl.png", |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    mag_filter: ImageFilterMode::Linear,
                    min_filter: ImageFilterMode::Linear,
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        });

    my_assets.textures.grass_roughness =
        assets.load_with_settings("textures/grass/roughness.png", |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    mag_filter: ImageFilterMode::Linear,
                    min_filter: ImageFilterMode::Linear,
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        });

    my_assets.textures.grass_occlusion =
        assets.load_with_settings("textures/grass/ambient_occlusion.png", |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    mag_filter: ImageFilterMode::Linear,
                    min_filter: ImageFilterMode::Linear,
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        });

    // images
    my_assets.imgs.select_border = assets.load("imgs/select_border.png");
    my_assets.imgs.cursor_relocate = assets.load("imgs/cursor/relocate.png");
    my_assets.imgs.cursor_select = assets.load("imgs/cursor/select.png");
    my_assets.imgs.cursor_standard = assets.load("imgs/cursor/standard.png");
    my_assets.imgs.cmd_intrfce_structures = assets.load("imgs/cmd_cntr_structures.png");
    my_assets.imgs.cmd_intrfce_units = assets.load("imgs/cmd_cntr_units.png");
    my_assets.imgs.cmd_intrfce_background = assets.load("imgs/cmd_interface/root_ctr.png");
    my_assets.imgs.structure_barracks = assets.load("imgs/structures/barracks.png");
    my_assets.imgs.structure_cannon = assets.load("imgs/structures/cannon.png");
    my_assets.imgs.structure_vehicle_depot = assets.load("imgs/structures/vehicle_depot.png");
    my_assets.imgs.structure_research_center = assets.load("imgs/structures/research_center.png");
    my_assets.imgs.structure_satellite_dish = assets.load("imgs/structures/satellite_dish.png");
    my_assets.imgs.unit_tank_gen1 = assets.load("imgs/units/tank_gen1.png");
    my_assets.imgs.unit_tank_gen2 = assets.load("imgs/units/tank_gen2.png");

    // units
    my_assets.models.tank_gen1 = assets.load("models/units/tank_gen1/tank.gltf#Scene0");
    my_assets.models.tank_gen2 = assets.load("models/units/tank_gen2/tank_gen2.gltf#Scene0");

    // structures
    my_assets.models.barracks = assets.load("models/structures/barracks.gltf#Scene0");
    my_assets.models.cannon = assets.load("models/structures/cannon.gltf#Scene0");
    my_assets.models.vehicle_depot =
        assets.load("models/structures/vehicle_depot/vehicle_depot.gltf#Scene0");
    my_assets.models.research_center = assets.load("models/structures/research_center.gltf#Scene0");
    my_assets.models.satellite_dish = assets.load("models/structures/satellite_dish.gltf#Scene0");

    // structure placeholders valid
    my_assets.models.placeholders.barracks_valid =
        assets.load("models/structures/placeholders/valid/barracks.gltf#Scene0");
    my_assets.models.placeholders.cannon_valid =
        assets.load("models/structures/placeholders/valid/cannon.gltf#Scene0");
    my_assets.models.placeholders.vehicle_depot_valid =
        assets.load("models/structures/placeholders/valid/vehicle_depot/vehicle_depot.gltf#Scene0");
    my_assets.models.placeholders.research_center_valid =
        assets.load("models/structures/placeholders/valid/research_center.gltf#Scene0");
    my_assets.models.placeholders.satellite_dish_valid =
        assets.load("models/structures/placeholders/valid/satellite_dish.gltf#Scene0");

    // structure placeholders invalid
    my_assets.models.placeholders.barracks_invalid =
        assets.load("models/structures/placeholders/invalid/barracks.gltf#Scene0");
    my_assets.models.placeholders.cannon_invalid =
        assets.load("models/structures/placeholders/invalid/cannon.gltf#Scene0");
    my_assets.models.placeholders.vehicle_depot_invalid = assets
        .load("models/structures/placeholders/invalid/vehicle_depot/vehicle_depot.gltf#Scene0");
    my_assets.models.placeholders.research_center_invalid =
        assets.load("models/structures/placeholders/invalid/research_center.gltf#Scene0");
    my_assets.models.placeholders.satellite_dish_invalid =
        assets.load("models/structures/placeholders/invalid/satellite_dish.gltf#Scene0");

    // audio
    my_assets.audio.place_structure = assets.load("audio/place_structure.ogg");
}
