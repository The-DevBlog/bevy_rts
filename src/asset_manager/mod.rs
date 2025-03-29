use bevy::prelude::*;

pub mod audio;

use audio::AudioPlugin;

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin);
    }
}
