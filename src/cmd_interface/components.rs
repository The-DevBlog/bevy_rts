use bevy::prelude::*;

use crate::components::units::UnitType;

#[derive(Component)]
pub struct CmdInterfaceCtr;

#[derive(Component)]
pub struct UnitCtr(pub UnitType);

#[derive(Component)]
pub struct InfoCtr;

#[derive(Component)]
pub struct InfoCtrCost;

#[derive(Component)]
pub struct InfoCtrName;
