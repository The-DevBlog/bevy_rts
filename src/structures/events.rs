use bevy::prelude::*;

use crate::units::components::UnitType;

#[derive(Event)]
pub struct SetPrimaryStructureEv(pub Entity);

#[derive(Event)]
pub struct DeselectAllStructuresEv;

#[derive(Event)]
pub struct BuildVehicleEv(pub UnitType);
