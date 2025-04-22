use bevy::{
    core_pipeline::{
        fxaa::{Fxaa, Sensitivity},
        prepass::{DepthPrepass, NormalPrepass},
        Skybox,
    },
    image::CompressedImageFormats,
    math::bounding::Aabb2d,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
    },
};
use bevy_kira_audio::SpatialAudioReceiver;
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};
use bevy_rts_pathfinding::components as pf_comps;

use crate::shaders::{
    outline::ShaderSettingsOutline, stylized::ShaderSettingsStylized, tint::ShaderSettingsTint,
};

use super::*;

const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    (
        "imgs/skybox/Ryfjallet_cubemap.png",
        CompressedImageFormats::NONE,
    ),
    (
        "imgs/skybox/textures/Ryfjallet_cubemap_astc4x4.ktx2",
        CompressedImageFormats::ASTC_LDR,
    ),
    (
        "imgs/skybox/Ryfjallet_cubemap_bc7.ktx2",
        CompressedImageFormats::BC,
    ),
    (
        "imgs/skybox/Ryfjallet_cubemap_etc2.ktx2",
        CompressedImageFormats::ETC2,
    ),
];

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (cycle_cubemap_asset, asset_loaded.after(cycle_cubemap_asset)),
            );
    }
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

fn spawn_camera(mut cmds: Commands, assets: Res<AssetServer>) {
    let skybox_handle = assets.load(CUBEMAPS[0].0);

    cmds.spawn((
        Camera3d::default(),
        ShaderSettingsTint::default(),
        ShaderSettingsStylized::default(),
        ShaderSettingsOutline::default(),
        DepthPrepass,
        NormalPrepass,
        Msaa::Off,
        Fxaa {
            enabled: true,
            edge_threshold: Sensitivity::Ultra,
            edge_threshold_min: Sensitivity::Ultra,
        },
        pf_comps::GameCamera,
        SpatialAudioReceiver,
        RtsCamera {
            bounds: Aabb2d::new(Vec2::ZERO, Vec2::new(MAP_WIDTH / 2.0, MAP_DEPTH / 2.0)),
            min_angle: 60.0f32.to_radians(),
            height_max: 300.0,
            height_min: 30.0,
            ..default()
        },
        RtsCameraControls {
            edge_pan_width: 0.01,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            pan_speed: 165.0,
            zoom_sensitivity: 0.2,
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            ..default()
        },
    ));

    cmds.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

const CUBEMAP_SWAP_DELAY: f32 = 3.0;

fn cycle_cubemap_asset(
    time: Res<Time>,
    mut next_swap: Local<f32>,
    mut cubemap: ResMut<Cubemap>,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
) {
    let now = time.elapsed_secs();
    if *next_swap == 0.0 {
        *next_swap = now + CUBEMAP_SWAP_DELAY;
        return;
    } else if now < *next_swap {
        return;
    }
    *next_swap += CUBEMAP_SWAP_DELAY;

    let supported_compressed_formats =
        CompressedImageFormats::from_features(render_device.features());

    let mut new_index = cubemap.index;
    for _ in 0..CUBEMAPS.len() {
        new_index = (new_index + 1) % CUBEMAPS.len();
        if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
            break;
        }
        // info!(
        //     "Skipping format which is not supported by current hardware: {:?}",
        //     CUBEMAPS[new_index]
        // );
    }

    // Skip swapping to the same texture. Useful for when ktx2, zstd, or compressed texture support
    // is missing
    if new_index == cubemap.index {
        return;
    }

    cubemap.index = new_index;
    cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
    cubemap.is_loaded = false;
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
        // info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
