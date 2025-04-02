use bevy::prelude::*;

pub mod structures;

#[derive(Component, Clone)]
pub struct UnitSelectBorder(pub Entity);

#[derive(Component)]
pub struct BorderSize(pub Vec2);
