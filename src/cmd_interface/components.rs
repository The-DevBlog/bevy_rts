use bevy::prelude::*;

#[derive(Component)]
pub struct CmdInterfaceCtr;

#[derive(Component)]
pub struct MiniMapCtr;

#[derive(Component)]
pub struct IconsCtr;

#[derive(Component)]
pub struct BuildColumnsCtr;

#[derive(Component)]
#[require(Button)]
pub struct BuildActionBtn;

#[derive(Component)]
pub struct Bldg;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct BuildStructurePlaceholder;
