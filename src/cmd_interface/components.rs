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

#[derive(Component, Clone, Copy)]
pub struct Structure(pub StructureType);

#[derive(Clone, Copy)]
pub enum StructureType {
    Red,
    Green,
    Blue,
    Black,
    White,
}

impl Structure {
    pub fn build(&self) -> (Cuboid, Color) {
        match self.0 {
            StructureType::Red => (Cuboid::new(25.0, 25.0, 25.0), Color::srgb(1.0, 0.0, 0.0)),
            StructureType::Green => (Cuboid::new(25.0, 25.0, 25.0), Color::srgb(0.0, 1.0, 0.0)),
            StructureType::Blue => (Cuboid::new(25.0, 25.0, 25.0), Color::srgb(0.0, 0.0, 1.0)),
            StructureType::Black => (Cuboid::new(25.0, 25.0, 25.0), Color::srgb(0.0, 0.0, 0.0)),
            StructureType::White => (Cuboid::new(25.0, 25.0, 25.0), Color::srgb(1.0, 1.0, 1.0)),
        }
    }
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct BuildStructurePlaceholder;
