use bevy::prelude::*;

#[derive(Event)]
pub struct SetBoxCoordsEv;

#[derive(Event)]
pub struct SetInitialBoxCoords;

#[derive(Event)]
pub struct ClearBoxCoordsEv;

#[derive(Event)]
pub struct DragSelectEv;

#[derive(Event)]
pub struct SetWorldCoordsEv;
