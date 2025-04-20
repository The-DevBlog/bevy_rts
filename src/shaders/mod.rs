use bevy::prelude::*;

pub mod outline;
pub mod stylized;
pub mod tint;

use outline::OutlineShaderPlugin;
use stylized::StylizedShaderPlugin;
use tint::TintShaderPlugin;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OutlineShaderPlugin, TintShaderPlugin, StylizedShaderPlugin));
    }
}
