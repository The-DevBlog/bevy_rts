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
    pub viewport: Viewport,
    pub world: World,
}

impl SelectBox {
    pub fn empty_coords(&mut self) {
        self.viewport.start_1 = Vec2::ZERO;
        self.viewport.start_2 = Vec2::ZERO;
        self.viewport.end_1 = Vec2::ZERO;
        self.viewport.end_2 = Vec2::ZERO;
        self.world.start_1 = Vec3::ZERO;
        self.world.start_2 = Vec3::ZERO;
        self.world.end_1 = Vec3::ZERO;
        self.world.end_2 = Vec3::ZERO;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Viewport {
    pub start_1: Vec2,
    pub start_2: Vec2,
    pub end_1: Vec2,
    pub end_2: Vec2,
}

#[derive(Default, Debug, Clone)]
pub struct World {
    pub start_1: Vec3,
    pub start_2: Vec3,
    pub end_1: Vec3,
    pub end_2: Vec3,
}

#[derive(Resource, Default, Debug)]
pub struct GameCommands {
    pub drag_select: bool,
    pub selected: bool,
}

fn setup(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
    my_assets.img_select_border = assets.load("imgs/select_border.png");
}
