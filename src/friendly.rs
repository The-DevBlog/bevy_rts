use bevy::prelude::*;
use bevy_rapier3d::{
    plugin::RapierContext,
    prelude::{ExternalImpulse, QueryFilter},
};

use super::components::*;
use super::resources::*;

pub struct FriendlyPlugin;

impl Plugin for FriendlyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_unit_destination, move_unit::<Friendly>));
    }
}

pub fn set_unit_destination(
    mouse_coords: ResMut<MouseCoords>,
    mut friendly_q: Query<(&mut Destination, &mut Target, &Transform, &Selected), With<Friendly>>,
    input: Res<ButtonInput<MouseButton>>,
    game_cmds: Res<GameCommands>,
    cursor: Res<CustomCursor>,
    mut cmds: Commands,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
) {
    if !input.just_released(MouseButton::Left) || game_cmds.drag_select {
        return;
    }

    let (cam, cam_trans) = cam_q.single();

    let Some(ray) = cam.viewport_to_world(cam_trans, mouse_coords.local) else {
        return;
    };

    let hit = rapier_context.cast_ray(
        ray.origin,
        ray.direction.into(),
        f32::MAX,
        true,
        QueryFilter::only_dynamic(),
    );

    if let Some(_) = hit {
        return;
    }

    for (mut friendly_destination, mut target, trans, selected) in friendly_q.iter_mut() {
        if !selected.0 {
            continue;
        }

        if cursor.state == CursorState::Relocate {
            target.0 = None;
        }

        let mut destination = mouse_coords.global;
        destination.y += trans.scale.y / 2.0; // calculate for entity height
        friendly_destination.0 = Some(destination);
        println!("Unit Moving to ({}, {})", destination.x, destination.y);
    }
}

fn move_unit<T: Component>(
    mut unit_q: Query<
        (
            &mut CurrentAction,
            &mut Transform,
            &mut ExternalImpulse,
            &Speed,
            &mut Destination,
            &Target,
        ),
        With<T>,
    >,
    target_transform_q: Query<&Transform, Without<T>>,
    time: Res<Time>,
) {
    for (mut action, mut trans, mut ext_impulse, speed, mut destination, target) in
        unit_q.iter_mut()
    {
        if let Some(target) = target.0 {
            if let Ok(target_transform) = target_transform_q.get(target) {
                let direction = (target_transform.translation - trans.translation).normalize();
                rotate_towards(&mut trans, direction);
            }
        }

        if let Some(new_pos) = destination.0 {
            let distance = new_pos - trans.translation;
            let direction = Vec3::new(distance.x, 0.0, distance.z).normalize();
            rotate_towards(&mut trans, direction);

            if distance.length_squared() <= 5.0 {
                destination.0 = None;
                action.0 = Action::None;
            } else {
                action.0 = Action::Relocate;
                ext_impulse.impulse += direction * speed.0 * time.delta_seconds();
            }
        }
    }
}

fn rotate_towards(trans: &mut Transform, direction: Vec3) {
    let target_yaw = direction.x.atan2(direction.z);
    trans.rotation = Quat::from_rotation_y(target_yaw);
}
