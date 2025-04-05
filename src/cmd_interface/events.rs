use bevy::prelude::*;

use crate::{structures::components::StructureType, units::components::UnitType};

// Event when user selects a structure to build (not actually placing the structure)
#[derive(Event)]
pub struct BuildStructureSelectEv(pub StructureType);

#[derive(Event)]
pub struct BuildUnitEv(pub UnitType);
