use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseCoords>()
            .init_resource::<BoxCoords>()
            .init_resource::<MouseClick>();
    }
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

#[derive(Resource)]
pub struct MouseClick {
    pub long_press_timer: Timer,
    pub long_press: bool,
    pub normal_press: bool,
}

impl Default for MouseClick {
    fn default() -> Self {
        MouseClick {
            long_press_timer: Timer::from_seconds(0.1, TimerMode::Once),
            long_press: false,
            normal_press: false,
        }
    }
}
