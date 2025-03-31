use bevy::image::*;
use bevy::prelude::*;

pub mod audio;

use audio::AudioPlugin;

use crate::resources::MyAssets;

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
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
    my_assets.imgs.unit_rifleman = assets.load("imgs/units/rifleman.png");
    my_assets.imgs.info_ctr = assets.load("imgs/cmd_interface/info_ctr.png");
    my_assets.imgs.info_ctr_dmg = assets.load("imgs/info_ctr/dmg.png");
    my_assets.imgs.info_ctr_speed = assets.load("imgs/info_ctr/speed.png");
    my_assets.imgs.info_ctr_build_time = assets.load("imgs/info_ctr/build_time.png");
    my_assets.imgs.info_ctr_hp = assets.load("imgs/info_ctr/hp.png");

    // units
    my_assets.models.rifleman = assets.load("models/units/tank_gen_1/tank_gen_1.gltf#Scene0"); // TODO: Temporary
    my_assets.models.tank_gen1 = assets.load("models/units/tank_gen_1/tank_gen_1.gltf#Scene0");
    my_assets.models.tank_gen2 = assets.load("models/units/tank_gen_2/tank_gen_2.gltf#Scene0");

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
}
