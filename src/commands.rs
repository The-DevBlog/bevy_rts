use bevy::{prelude::*, window::PrimaryWindow};

use crate::units::{Speed, Unit};

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_unit);
    }
}

// fn cursor_position(
//     window_q: Query<&Window, With<PrimaryWindow>>,
//     input: Res<ButtonInput<MouseButton>>,
// ) {
//     if input.just_pressed(MouseButton::Left) {
//         if let Some(pos) = window_q.single().cursor_position() {
//             println!("Cursor Position: {:?}", pos);
//         }
//     }
// }
fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}
fn move_unit(
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mut unit_q: Query<(&mut Transform, &Speed), With<Unit>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Left) {
        // for (mut transform, speed) in unit_q.iter_mut() {
        let (mut unit_trans, unit_speed) = unit_q.single_mut();
        let cursor_pos = window_q.single().cursor_position().unwrap();
        let (cam, cam_trans) = cam_q.single();

        let Some(new_pos) = cam.viewport_to_world_2d(cam_trans, cursor_pos) else {
            return;
        };

        unit_trans.translation.x = new_pos.x;
        unit_trans.translation.z = new_pos.y;

        println!("Position: {:?}", new_pos);
    }
}
