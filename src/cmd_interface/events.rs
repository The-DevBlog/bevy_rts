use bevy::prelude::*;

use super::components::Structure;

#[derive(Event)]
pub struct BuildStructureSelectEv(pub Structure);

#[derive(Event)]
pub struct BuildUnitEv;
