use bevy::prelude::*;

use crate::components::structures::Structure;

// Event when user selects a structure to build (not actually placing the structure)
#[derive(Event)]
pub struct BuildStructureSelectEv(pub Structure);

#[derive(Event)]
pub struct BuildUnitEv;
