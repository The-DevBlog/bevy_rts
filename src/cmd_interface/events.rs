use bevy::prelude::*;

use crate::components::{structures::StructureType, units::UnitType};

// Event when user selects a structure to build (not actually placing the structure)
#[derive(Event)]
pub struct BuildStructureSelectEv(pub StructureType);

#[derive(Event)]
pub struct BuildUnitEv(pub UnitType);

#[derive(Event)]
pub struct ToggleInfoCtrEv;
