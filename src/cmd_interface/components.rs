use bevy::prelude::*;

use crate::units::components::UnitType;

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
pub struct InfoCtrDmgTxt;

#[derive(Component)]
pub struct InfoCtrBuildTime;

#[derive(Component)]
pub struct InfoCtrBuildTimeTxt;

#[derive(Component)]
pub struct InfoCtrSpeed;

#[derive(Component)]
pub struct InfoCtrSpeedTxt;

#[derive(Component)]
pub struct InfoCtrHp;

#[derive(Component)]
pub struct InfoCtrHpTxt;

#[derive(Component)]
pub struct UnitBuildColumn;

#[derive(Component)]
pub struct BankCtr;
