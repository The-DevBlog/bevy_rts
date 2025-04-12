use bevy::prelude::*;

pub mod outline_shader;

use outline_shader::OutlineShaderPlugin;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OutlineShaderPlugin);
    }
}
