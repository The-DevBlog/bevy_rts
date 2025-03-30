use bevy::prelude::*;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StructuresBuilt>();
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
