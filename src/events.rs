use bevy::prelude::*;
use bevy_rts_pathfinding::components::RtsObjSize;

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
pub struct SelectMultipleUnitEv;

#[derive(Event)]
pub struct DeselectAllEv;

#[derive(Event)]
pub struct SelectStructureEv(pub Entity);
