use bevy::prelude::*;

use crate::components::units::UnitType;

#[derive(Event)]
pub struct BuildVehicle(pub UnitType);

#[derive(Event)]
pub struct BuildSoldier(pub UnitType);
