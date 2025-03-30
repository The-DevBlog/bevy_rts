use bevy::prelude::*;

pub mod structures;
pub mod units;

#[derive(Component, Clone)]
pub struct SelectBorder(pub Entity);

#[derive(Component)]
pub struct BorderSize(pub Vec2);
