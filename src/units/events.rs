use bevy::prelude::*;

use super::components::UnitType;

#[derive(Event)]
pub struct BuildVehicleEv(pub UnitType);

#[derive(Event)]
pub struct BuildSoldierEv(pub UnitType);
