use bevy::prelude::*;

#[derive(Event)]
pub struct SetBoxCoordsEv;

#[derive(Event)]
pub struct SetStartBoxCoordsEv;

#[derive(Event)]
pub struct ClearBoxCoordsEv;

#[derive(Event)]
pub struct HandleDragSelectEv;

#[derive(Event)]
pub struct SetDragSelectEv;

#[derive(Event)]
pub struct SetUnitDestinationEv;

#[derive(Event)]
pub struct SelectSingleUnitEv(pub Entity);

#[derive(Event)]
pub struct DeselectAllEv;
