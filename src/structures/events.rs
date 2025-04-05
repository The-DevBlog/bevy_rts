use bevy::prelude::*;

#[derive(Event)]
pub struct SetPrimaryStructureEv(pub Entity);

#[derive(Event)]
pub struct DeselectAllStructuresEv;
