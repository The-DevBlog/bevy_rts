use bevy::prelude::*;

use super::components::UnitType;

#[derive(Event)]
pub struct QueueVehicleEv(pub UnitType);

#[derive(Event)]
pub struct QueueSolderEv(pub UnitType);
