use bevy::prelude::*;

use super::components::UnitType;

#[derive(Event)]
pub struct BuildVehicle(pub UnitType);

#[derive(Event)]
pub struct BuildSoldier(pub UnitType);
