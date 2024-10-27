use bevy::prelude::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Event)]
pub struct SetBoxCoordsEv;

#[derive(Event)]
pub struct SetInitialBoxCoords;

#[derive(Event)]
pub struct ClearBoxCoordsEv;

#[derive(Event)]
pub struct SetDragSelectEv;
