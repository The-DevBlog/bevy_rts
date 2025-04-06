use bevy::prelude::*;

use crate::units::components::UnitType;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StructuresBuilt>()
            .init_resource::<VehicleBuildQueue>();
    }
}

#[derive(Resource, Default, Debug)]
pub struct StructuresBuilt {
    pub barracks: u32,
    pub cannon: u32,
    pub vehicle_depot: u32,
    pub research_center: u32,
    pub satellite_dish: u32,
}

#[derive(Resource, Default)]
pub struct VehicleBuildQueue(pub Vec<(UnitType, Timer)>);
