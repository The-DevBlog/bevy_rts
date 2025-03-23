use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rts_pathfinding::components::{self as pf_comps};
use strum_macros::EnumIter;

use crate::{components::units::UnitType, resources::MyAssets};

#[derive(Component)]
pub struct CmdInterfaceCtr;

#[derive(Component)]
pub struct UnitCtr(pub UnitType);
