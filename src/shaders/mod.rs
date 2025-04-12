use bevy::prelude::*;

pub mod outline_shader;
pub mod tint_shader;

use outline_shader::OutlineShaderPlugin;
use tint_shader::TintShaderPlugin;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OutlineShaderPlugin, TintShaderPlugin));
    }
}
