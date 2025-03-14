use bevy::prelude::*;

mod build_actions;
pub mod components;
pub mod events;
pub mod ui;

use build_actions::BuildActionsPlugin;
use ui::UiPlugin;

pub struct CmdInterfacePlugin;

impl Plugin for CmdInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, BuildActionsPlugin));
    }
}
