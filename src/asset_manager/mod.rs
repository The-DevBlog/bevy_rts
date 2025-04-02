use bevy::prelude::*;

pub mod audio;
pub mod imgs;
pub mod models;
pub mod textures;

use audio::AudioPlugin;
use imgs::ImgsPlugin;
use models::ModelsPlugin;
use textures::TexturesPlugin;

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, ModelsPlugin, ImgsPlugin, TexturesPlugin));
    }
}
