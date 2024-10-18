use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {}
}

pub enum UnitType {
    Soldier,
    Tank,
}
