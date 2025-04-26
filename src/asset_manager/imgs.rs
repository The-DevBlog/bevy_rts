use bevy::prelude::*;

pub struct ImgsPlugin;

impl Plugin for ImgsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyImgs>()
            .add_systems(PreStartup, load_imgs);
    }
}

#[derive(Resource, Default)]
pub struct MyImgs {
    pub select_border: Handle<Image>,
    pub cursor_relocate: Handle<Image>,
    pub cursor_select: Handle<Image>,
    pub cursor_standard: Handle<Image>,
    pub cmd_intrfce_structures: Handle<Image>,
    pub cmd_intrfce_units: Handle<Image>,
    pub cmd_intrfce_mini_map: Handle<Image>,
    pub cmd_intrfce_funds: Handle<Image>,
    pub cmds_intrfce_build_columns_ctr: Handle<Image>,
    pub info_ctr: Handle<Image>,
    pub info_ctr_dmg: Handle<Image>,
    pub info_ctr_speed: Handle<Image>,
    pub info_ctr_build_time: Handle<Image>,
    pub info_ctr_hp: Handle<Image>,
    pub structure_barracks: Handle<Image>,
    pub structure_cannon: Handle<Image>,
    pub structure_vehicle_depot: Handle<Image>,
    pub structure_research_center: Handle<Image>,
    pub structure_satellite_dish: Handle<Image>,
    pub unit_tank_gen_1: Handle<Image>,
    pub unit_tank_gen_2: Handle<Image>,
    pub unit_artillery: Handle<Image>,
    pub unit_rifleman: Handle<Image>,
}

fn load_imgs(mut my_imgs: ResMut<MyImgs>, assets: Res<AssetServer>) {
    my_imgs.select_border = assets.load("imgs/select_border.png");

    my_imgs.cursor_relocate = assets.load("imgs/cursor/relocate.png");
    my_imgs.cursor_select = assets.load("imgs/cursor/select.png");
    my_imgs.cursor_standard = assets.load("imgs/cursor/standard.png");

    my_imgs.cmd_intrfce_structures = assets.load("imgs/cmd_cntr_structures.png");
    my_imgs.cmd_intrfce_units = assets.load("imgs/cmd_cntr_units.png");
    my_imgs.cmd_intrfce_mini_map = assets.load("imgs/cmd_interface/mini_map_ctr.png");
    my_imgs.cmd_intrfce_funds = assets.load("imgs/cmd_interface/funds_ctr.png");
    my_imgs.cmds_intrfce_build_columns_ctr =
        assets.load("imgs/cmd_interface/build_columns_ctr.png");

    my_imgs.structure_barracks = assets.load("imgs/structures/barracks.png");
    my_imgs.structure_cannon = assets.load("imgs/structures/cannon.png");
    my_imgs.structure_vehicle_depot = assets.load("imgs/structures/vehicle_depot.png");
    my_imgs.structure_research_center = assets.load("imgs/structures/research_center.png");
    my_imgs.structure_satellite_dish = assets.load("imgs/structures/satellite_dish.png");

    my_imgs.unit_tank_gen_1 = assets.load("imgs/units/tank_gen_1.png");
    my_imgs.unit_tank_gen_2 = assets.load("imgs/units/tank_gen_2.png");
    my_imgs.unit_artillery = assets.load("imgs/units/artillery.png");
    my_imgs.unit_rifleman = assets.load("imgs/units/rifleman.png");

    my_imgs.info_ctr = assets.load("imgs/cmd_interface/info_ctr.png");
    my_imgs.info_ctr_dmg = assets.load("imgs/info_ctr/dmg.png");
    my_imgs.info_ctr_speed = assets.load("imgs/info_ctr/speed.png");
    my_imgs.info_ctr_build_time = assets.load("imgs/info_ctr/build_time.png");
    my_imgs.info_ctr_hp = assets.load("imgs/info_ctr/hp.png");
}
