use bevy::math::bounding::Aabb2d;
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};
use bevy_rts_pathfinding::components as pf_comps;

use crate::{cmd_interface::components::StructurePlaceholder, resources::GameCommands};

use super::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, stop_scroll);
    }
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3d::default(),
        pf_comps::GameCamera,
        RtsCamera {
            bounds: Aabb2d::new(Vec2::ZERO, Vec2::new(MAP_WIDTH / 2.0, MAP_DEPTH / 2.0)),
            min_angle: 60.0f32.to_radians(),
            height_max: 200.0,
            ..default()
        },
        RtsCameraControls {
            edge_pan_width: 0.01,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            pan_speed: 165.0,
            zoom_sensitivity: 0.2,
            ..default()
        },
    ));
}

// prevent scrolling when hovering over the command interface or when placing a structure
fn stop_scroll(
    game_cmds: Res<GameCommands>,
    q_placeholder: Query<&StructurePlaceholder>,
    mut q_cam: Query<&mut RtsCameraControls>,
) {
    let Ok(mut cam_ctrls) = q_cam.get_single_mut() else {
        return;
    };

    if game_cmds.hvr_cmd_interface || !q_placeholder.is_empty() {
        cam_ctrls.zoom_sensitivity = 0.0;
    } else {
        cam_ctrls.zoom_sensitivity = 0.2;
    }
}
