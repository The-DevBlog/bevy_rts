use bevy::prelude::*;

use crate::components::structures::StructureType;

// Event when user selects a structure to build (not actually placing the structure)
#[derive(Event)]
pub struct BuildStructureSelectEv(pub StructureType);

#[derive(Event)]
pub struct BuildUnitEv;
