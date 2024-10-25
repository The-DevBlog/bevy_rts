use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseCoords>()
            .init_resource::<SelectBox>()
            .init_resource::<GameCommands>()
            .init_resource::<MyAssets>()
            .add_systems(PreStartup, setup);
    }
}

#[derive(Resource, Default)]
pub struct MyAssets {
    pub img_select_border: Handle<Image>,
}

#[derive(Resource, Default, Debug)]
pub struct MouseCoords {
    pub world: Vec3,
    pub viewport: Vec2,
}

#[derive(Resource, Default, Debug)]
pub struct SelectBox {
    pub coords_viewport: ViewportCoords,
    pub coords_world: WorldCoords,
}

impl SelectBox {
    pub fn empty_coords(&mut self) {
        self.coords_viewport.upper_1 = Vec2::ZERO;
        self.coords_viewport.upper_2 = Vec2::ZERO;
        self.coords_viewport.lower_1 = Vec2::ZERO;
        self.coords_viewport.lower_2 = Vec2::ZERO;
        self.coords_world.upper_1 = Vec3::ZERO;
        self.coords_world.upper_2 = Vec3::ZERO;
        self.coords_world.lower_1 = Vec3::ZERO;
        self.coords_world.lower_2 = Vec3::ZERO;
    }
}

#[derive(Default, Debug, Clone)]
pub struct ViewportCoords {
    pub upper_1: Vec2,
    pub upper_2: Vec2,
    pub lower_1: Vec2,
    pub lower_2: Vec2,
}

#[derive(Default, Debug, Clone)]
pub struct WorldCoords {
    pub upper_1: Vec3,
    pub upper_2: Vec3,
    pub lower_1: Vec3,
    pub lower_2: Vec3,
}

#[derive(Resource, Default, Debug)]
pub struct GameCommands {
    pub drag_select: bool,
    pub selected: bool,
}

fn setup(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
    my_assets.img_select_border = assets.load("imgs/select_border.png");
}
