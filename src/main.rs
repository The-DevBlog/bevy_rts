mod new;
mod original;

use bevy_mod_billboard::plugin::BillboardPlugin;
use new::NewPlugin;
use original::OriginalPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::new(),
            NewPlugin,
            // BillboardPlugin,
            // OriginalPlugin,
        ))
        .run();
}
