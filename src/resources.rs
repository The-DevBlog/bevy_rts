use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseCoords>()
            .init_resource::<SelectionBoxCoords>()
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
pub struct SelectionBoxCoords {
    pub world_start: Vec3,
    pub world_end: Vec3,
    pub viewport_start: Vec2,
    pub viewport_end: Vec2,
}

impl SelectionBoxCoords {
    pub fn empty(&mut self) {
        self.viewport_start = Vec2::ZERO;
        self.viewport_end = Vec2::ZERO;
        self.world_start = Vec3::ZERO;
        self.world_end = Vec3::ZERO;
    }
}

#[derive(Resource, Default, Debug)]
pub struct GameCommands {
    pub drag_select: bool,
    pub selected: bool,
}

fn setup(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
    my_assets.img_select_border = assets.load("imgs/select_border.png");
}
