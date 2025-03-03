use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use super::events::*;
use crate::utils;
use bevy_rts_pathfinding::components::{self as pf_comps};

const CLR_BUILD_ACTIONS_BACKGROUND: Color = Color::srgb(0.07, 0.07, 0.07);
const CLR_BUILD_ACTIONS_BACKGROUND_HOVER: Color = Color::srgb(0.2, 0.2, 0.2);

pub struct BuildActionsPlugin;

impl Plugin for BuildActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_build_action_btn_interaction,
                sync_placeholder,
                cancel_build_structure,
            ),
        )
        .add_observer(handle_build_structure);
    }
}

fn handle_build_action_btn_interaction(
    mut cmds: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut q_btn_bldg: Query<(&Interaction, &mut BackgroundColor), With<Bldg>>,
    mut q_btn_unit: Query<(&Interaction, &mut BackgroundColor), (With<Unit>, Without<Bldg>)>,
) {
    for (interaction, mut border_clr) in q_btn_bldg.iter_mut() {
        match interaction {
            Interaction::None => border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND,
            Interaction::Pressed => {
                border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND_HOVER;
                if input.just_pressed(MouseButton::Left) {
                    println!("Building Structure!");
                    cmds.trigger(BuildStructureEv);
                }
            }
            _ => border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND_HOVER,
        }
    }

    for (interaction, mut border_clr) in q_btn_unit.iter_mut() {
        match interaction {
            Interaction::None => border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND,
            Interaction::Pressed => {
                border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND_HOVER;
                if input.just_pressed(MouseButton::Left) {
                    println!("Building Unit!");
                    cmds.trigger(BuildUnitEv);
                }
            }
            _ => border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND_HOVER,
        }
    }
}

fn cancel_build_structure(
    q_placeholder: Query<Entity, With<BuildStructurePlaceholder>>,
    mut cmds: Commands,
    input: Res<ButtonInput<MouseButton>>,
) {
    if q_placeholder.is_empty() {
        return;
    }

    if input.just_pressed(MouseButton::Right) {
        for placeholder_ent in q_placeholder.iter() {
            cmds.entity(placeholder_ent).despawn_recursive();
        }
    }
}

fn handle_build_structure(
    _trigger: Trigger<BuildStructureEv>,
    q_placeholder: Query<Entity, With<BuildStructurePlaceholder>>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for placeholder_ent in q_placeholder.iter() {
        cmds.entity(placeholder_ent).despawn_recursive();
    }

    let bldg = (
        BuildStructurePlaceholder,
        Transform::default(),
        Mesh3d(meshes.add(Cuboid::new(25.0, 25.0, 25.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.27, 0.36, 0.46))),
    );

    cmds.spawn(bldg);
}

fn sync_placeholder(
    mut q_placeholder: Query<&mut Transform, With<BuildStructurePlaceholder>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<pf_comps::GameCamera>>,
    map_base_q: Query<&GlobalTransform, With<pf_comps::MapBase>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut transform) = q_placeholder.get_single_mut() else {
        return;
    };

    let (cam, cam_trans) = cam_q.single();

    let Some(viewport_cursor) = window_q.single().cursor_position() else {
        return;
    };

    let coords = utils::get_world_coords(map_base_q.single(), &cam_trans, &cam, viewport_cursor);

    if let Some(coords) = coords {
        transform.translation = coords;
    }
}
