use bevy::image::*;
use bevy::prelude::*;

// TODO: Im currently not using this at all, but im leaving it in place for future reference
pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyTextures>()
            .add_systems(PreStartup, load_texures);
    }
}

#[derive(Resource, Default)]
pub struct MyTextures {
    pub grass_clr: Handle<Image>,
    pub grass_normal: Handle<Image>,
    pub grass_roughness: Handle<Image>,
    pub grass_occlusion: Handle<Image>,
}

fn load_texures(mut my_textures: ResMut<MyTextures>, assets: Res<AssetServer>) {
    my_textures.grass_clr = assets.load_with_settings("textures/grass/color.png", |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            }),
            ..default()
        }
    });

    my_textures.grass_normal =
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

    my_textures.grass_roughness =
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

    my_textures.grass_occlusion =
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
