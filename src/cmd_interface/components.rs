use bevy::prelude::*;

use crate::components::units::UnitType;

#[derive(Component)]
pub struct CmdInterfaceCtr;

#[derive(Component)]
pub struct UnitCtr(pub UnitType);
