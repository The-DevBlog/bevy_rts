use bevy::prelude::*;

pub struct ModelsPlugin;

impl Plugin for ModelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyModels>()
            .add_systems(PreStartup, load_models);
    }
}

#[derive(Resource, Default)]
pub struct MyModels {
    pub barracks: Handle<Scene>,
    pub tank_gen1: Handle<Scene>,
    pub tank_gen2: Handle<Scene>,
    pub rifleman: Handle<Scene>,
    pub cannon: Handle<Scene>,
    pub vehicle_depot: Handle<Scene>,
    pub research_center: Handle<Scene>,
    pub satellite_dish: Handle<Scene>,
    pub placeholders: Placeholders,
}

#[derive(Default)]
pub struct Placeholders {
    pub barracks_valid: Handle<Scene>,
    pub barracks_invalid: Handle<Scene>,
    pub cannon_valid: Handle<Scene>,
    pub cannon_invalid: Handle<Scene>,
    pub vehicle_depot_valid: Handle<Scene>,
    pub vehicle_depot_invalid: Handle<Scene>,
    pub research_center_valid: Handle<Scene>,
    pub research_center_invalid: Handle<Scene>,
    pub satellite_dish_valid: Handle<Scene>,
    pub satellite_dish_invalid: Handle<Scene>,
}

fn load_models(mut my_models: ResMut<MyModels>, assets: Res<AssetServer>) {
    // units
    my_models.rifleman = assets.load("models/units/tank_gen_1/tank_gen_1.gltf#Scene0"); // TODO: Temporary
    my_models.tank_gen1 = assets.load("models/units/tank_gen_1/tank_gen_1.gltf#Scene0");
    my_models.tank_gen2 = assets.load("models/units/tank_gen_2/tank_gen_2.gltf#Scene0");

    // structures
    my_models.barracks = assets.load("models/structures/barracks.gltf#Scene0");
    my_models.cannon = assets.load("models/structures/cannon.gltf#Scene0");
    my_models.vehicle_depot =
        assets.load("models/structures/vehicle_depot/vehicle_depot.gltf#Scene0");
    my_models.research_center = assets.load("models/structures/research_center.gltf#Scene0");
    my_models.satellite_dish = assets.load("models/structures/satellite_dish.gltf#Scene0");

    // structure placeholders valid
    my_models.placeholders.barracks_valid =
        assets.load("models/structures/placeholders/valid/barracks.gltf#Scene0");
    my_models.placeholders.cannon_valid =
        assets.load("models/structures/placeholders/valid/cannon.gltf#Scene0");
    my_models.placeholders.vehicle_depot_valid =
        assets.load("models/structures/placeholders/valid/vehicle_depot/vehicle_depot.gltf#Scene0");
    my_models.placeholders.research_center_valid =
        assets.load("models/structures/placeholders/valid/research_center.gltf#Scene0");
    my_models.placeholders.satellite_dish_valid =
        assets.load("models/structures/placeholders/valid/satellite_dish.gltf#Scene0");

    // structure placeholders invalid
    my_models.placeholders.barracks_invalid =
        assets.load("models/structures/placeholders/invalid/barracks.gltf#Scene0");
    my_models.placeholders.cannon_invalid =
        assets.load("models/structures/placeholders/invalid/cannon.gltf#Scene0");
    my_models.placeholders.vehicle_depot_invalid = assets
        .load("models/structures/placeholders/invalid/vehicle_depot/vehicle_depot.gltf#Scene0");
    my_models.placeholders.research_center_invalid =
        assets.load("models/structures/placeholders/invalid/research_center.gltf#Scene0");
    my_models.placeholders.satellite_dish_invalid =
        assets.load("models/structures/placeholders/invalid/satellite_dish.gltf#Scene0");
}
