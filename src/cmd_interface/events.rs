use bevy::prelude::*;

use crate::components::structures::Structure;

// Event when user selects a structure to build (not actually placing the structure)
#[derive(Event)]
pub struct BuildStructureSelectEv(pub Structure);

#[derive(Event)]
pub struct BuildUnitEv;

#[derive(Event)]
pub struct ChangeInfoCtrEv {
    pub name: String,
    pub cost: i32,
}

impl ChangeInfoCtrEv {
    pub fn new(name: String, cost: i32) -> Self {
        Self { name, cost }
    }
}
