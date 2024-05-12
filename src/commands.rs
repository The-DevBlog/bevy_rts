use std::f32::EPSILON;

use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::ExternalImpulse, pipeline::QueryFilter, plugin::RapierContext,
    render::ColliderDebugColor,
};

use crate::{
    resources::{BoxCoords, MouseClick, MouseCoords},
    Selected, Speed, TargetPos, Unit,
};

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                drag_select,
                deselect,
                move_unit,
                set_unit_destination,
                single_select,
            ),
        );
    }
}

fn set_unit_destination(
    mouse_coords: ResMut<MouseCoords>,
    mut unit_q: Query<(&mut TargetPos, &Transform), With<Selected>>,
    mouse_click: ResMut<MouseClick>,
) {
    if mouse_click.normal_press {
        for (mut unit_target_pos, trans) in unit_q.iter_mut() {
            let mut destination = mouse_coords.global;
            destination.y += trans.scale.y / 2.0; // calculate for entity height
            unit_target_pos.0 = Some(destination);
            println!(
                "global mouse: {:?}, unit: {:?}",
                mouse_coords.local,
                unit_target_pos.0.unwrap()
            );
            println!("Unit Moving");
        }
    }
}

fn move_unit(
    mut unit_q: Query<(&mut Transform, &mut ExternalImpulse, &Speed, &mut TargetPos), With<Unit>>,
    time: Res<Time>,
) {
    for (trans, mut ext_impulse, speed, mut target_pos) in unit_q.iter_mut() {
        if let Some(new_pos) = target_pos.0 {
            let distance = new_pos - trans.translation;
            if distance.length_squared() <= (speed.0 * time.delta_seconds()).powi(2) + EPSILON {
                target_pos.0 = None;
                println!("Unit Stopping");
            } else {
                ext_impulse.impulse += distance.normalize() * speed.0 * time.delta_seconds();
            }
        }
    }
}

pub fn deselect(
    mut cmds: Commands,
    mut select_q: Query<Entity, With<Selected>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Right) {
        for entity in select_q.iter_mut() {
            println!("Unit deselected");
            cmds.entity(entity).insert(ColliderDebugColor(Color::NONE));
            cmds.entity(entity).remove::<Selected>();
        }
    }
}

pub fn drag_select(
    mut cmds: Commands,
    mut gizmos: Gizmos,
    unit_q: Query<(Entity, &Transform), With<Unit>>,
    box_coords: Res<BoxCoords>,
    mouse_click: Res<MouseClick>,
) {
    if !mouse_click.long_press {
        return;
    }

    let start = box_coords.global_start;
    let end = box_coords.global_end;

    // draw rectangle
    gizmos.line(start, Vec3::new(end.x, 0.0, start.z), Color::GRAY);
    gizmos.line(start, Vec3::new(start.x, 0.0, end.z), Color::GRAY);
    gizmos.line(Vec3::new(start.x, 0.0, end.z), end, Color::GRAY);
    gizmos.line(Vec3::new(end.x, 0.0, start.z), end, Color::GRAY);

    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let min_z = start.z.min(end.z);
    let max_z = start.z.max(end.z);

    for (unit_ent, unit_trans) in unit_q.iter() {
        // check to see if units are within selection rectangle
        let unit_pos = unit_trans.translation;
        let in_box_bounds = unit_pos.x >= min_x
            && unit_pos.x <= max_x
            && unit_pos.z >= min_z
            && unit_pos.z <= max_z;

        if in_box_bounds {
            cmds.entity(unit_ent)
                .insert((ColliderDebugColor(Color::GREEN), Selected));
        } else {
            cmds.entity(unit_ent)
                .remove::<Selected>()
                .insert(ColliderDebugColor(Color::NONE));
        }
    }
}

pub fn single_select(
    mut cmds: Commands,
    rapier_context: Res<RapierContext>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    select_q: Query<(Entity, &Selected)>,
    mouse_coords: Res<MouseCoords>,
    mouse_click: ResMut<MouseClick>,
) {
    if !mouse_click.normal_press {
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

    if let Some((ent, _toi)) = hit {
        // if unit is already selected, remove Selected component
        if let Ok((_, _)) = select_q.get(ent) {
            cmds.entity(ent)
                .remove::<Selected>()
                .insert(ColliderDebugColor(Color::NONE));
        // else add the Selected component
        } else {
            cmds.entity(ent)
                .insert((ColliderDebugColor(Color::GREEN), Selected));
        }
    }
}
