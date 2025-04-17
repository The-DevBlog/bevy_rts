use bevy::prelude::*;

pub mod outline;
pub mod tint_shader;

use outline::OutlineShaderPlugin;
use tint_shader::TintShaderPlugin;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OutlineShaderPlugin));
    }
}
