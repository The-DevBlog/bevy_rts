use bevy::prelude::*;
use std::collections::HashMap;

use crate::units::components::UnitType;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InfoContainerData>()
            .init_resource::<BuildQueueCount>();
    }
}

#[derive(Resource, Default)]
pub struct InfoContainerData {
    pub active: bool,
    pub name: String,
    pub cost: i32,
    pub build_time: u64,
    pub hp: Option<i32>,
    pub dmg: Option<i32>,
    pub speed: Option<f32>,
}

#[derive(Resource, Default)]
pub struct BuildQueueCount(pub HashMap<UnitType, usize>);

impl BuildQueueCount {
    pub fn add(&mut self, unit: &UnitType) {
        *self.0.entry(*unit).or_insert(0) += 1;
    }

    pub fn remove(&mut self, unit: &UnitType) {
        *self.0.entry(*unit).or_insert(0) -= 1;

        // if let Some(count) = self.0.get_mut(unit) {
        //     if *count > 0 {
        //         *count -= 1;
        //     }
        // }
    }

    pub fn get(&self, unit: &UnitType) -> usize {
        *self.0.get(unit).unwrap_or(&0)
    }
}
