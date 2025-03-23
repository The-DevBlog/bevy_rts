use bevy::prelude::*;

use crate::components::structures::Structure;

#[derive(Event)]
pub struct BuildStructureSelectEv(pub Structure);

#[derive(Event)]
pub struct BuildUnitEv;
