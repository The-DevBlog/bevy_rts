use bevy::image::*;
use bevy::prelude::*;

pub mod audio;
pub mod imgs;
pub mod models;

use audio::AudioPlugin;
use imgs::ImgsPlugin;
use models::ModelsPlugin;

use crate::resources::MyImgs;

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, ModelsPlugin, ImgsPlugin))
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut my_assets: ResMut<MyImgs>, assets: Res<AssetServer>) {
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
}
