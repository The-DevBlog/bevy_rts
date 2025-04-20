use bevy::prelude::*;

pub mod outline;
pub mod outline_2;
pub mod stylized;
pub mod tint;

use outline::OutlineShaderPlugin;
use outline_2::Outline2ShaderPlugin;
use stylized::StylizedShaderPlugin;
use tint::TintShaderPlugin;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Outline2ShaderPlugin, TintShaderPlugin, StylizedShaderPlugin));
    }
}
