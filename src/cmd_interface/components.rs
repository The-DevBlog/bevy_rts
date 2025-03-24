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

#[derive(Component)]
pub struct InfoCtrDmg;

#[derive(Component)]
pub struct InfoCtrBuildTime;

#[derive(Component)]
pub struct InfoCtrSpeed;

#[derive(Component)]
pub struct InfoCtrHP;
