use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseCoords>()
            .init_resource::<BoxCoords>()
            .init_resource::<GameCommands>()
            .init_resource::<CustomCursor>()
            .init_resource::<MyAssets>()
            .add_systems(Startup, setup);
    }
}

#[derive(Resource, Default)]
pub struct MyAssets {
    pub img_select_border: Handle<Image>,
}

#[derive(Resource, Default, Debug)]
pub struct MouseCoords {
    pub global: Vec3,
    pub local: Vec2,
}

#[derive(Resource, Default, Debug)]
pub struct BoxCoords {
    pub global_start: Vec3,
    pub global_end: Vec3,
    pub local_start: Vec2,
    pub local_end: Vec2,
}

impl BoxCoords {
    pub fn empty_global(&mut self) {
        self.global_start = Vec3::ZERO;
        self.global_end = Vec3::ZERO;
    }
}

#[derive(Resource, Default, Debug)]
pub struct GameCommands {
    pub drag_select: bool,
    pub single_select: bool,
    pub selected: bool,
}

#[derive(Resource)]
pub struct CustomCursor {
    pub state: CursorState,
}

#[derive(PartialEq, Debug)]
pub enum CursorState {
    Relocate,
    Normal,
}

impl Default for CustomCursor {
    fn default() -> Self {
        CustomCursor {
            state: CursorState::Normal,
        }
    }
}

fn setup(mut my_assets: ResMut<MyAssets>, assets: Res<AssetServer>) {
    my_assets.img_select_border = assets.load("imgs/select_border.png");
}
