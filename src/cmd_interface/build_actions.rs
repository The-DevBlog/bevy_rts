use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::Collider;

use super::components::*;
use super::events::*;
use crate::events::DeselectAllEv;
use crate::resources::CursorState;
use crate::resources::DbgOptions;
use crate::resources::GameCommands;
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
                cmd_interface_interaction,
                build_structure_btn_interaction,
                build_unit_btn_interaction,
                sync_placeholder,
                build_structure,
                cancel_build_structure,
            ),
        )
        .add_observer(select_structure);
    }
}

fn cmd_interface_interaction(
    mut game_cmds: ResMut<GameCommands>,
    q_p: Query<&Interaction, With<CmdInterfaceCtr>>,
    q_c: Query<&Interaction, Or<(With<Structure>, With<Unit>)>>,
) {
    let hvr_parent = q_p.iter().any(|intrct| *intrct == Interaction::Hovered);
    let hvr_child = q_c.iter().any(|intrct| *intrct == Interaction::Hovered);
    let click_parent = q_p.iter().any(|intrct| *intrct == Interaction::Pressed);
    let click_child = q_c.iter().any(|intrct| *intrct == Interaction::Pressed);

    game_cmds.hvr_cmd_interface = hvr_parent || hvr_child || click_parent || click_child;
}

fn build_structure_btn_interaction(
    mut cmds: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut q_btn_bldg: Query<(&Interaction, &mut BackgroundColor, &Structure), With<Structure>>,
) {
    for (interaction, mut border_clr, structure) in q_btn_bldg.iter_mut() {
        match interaction {
            Interaction::None => border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND,
            Interaction::Pressed => {
                border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND_HOVER;
                if input.just_pressed(MouseButton::Left) {
                    cmds.trigger(BuildStructureSelectEv(structure.clone()));
                }
            }
            _ => border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND_HOVER,
        }
    }
}

fn build_unit_btn_interaction(
    mut cmds: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut q_btn_unit: Query<(&Interaction, &mut BackgroundColor), With<Unit>>,
) {
    for (interaction, mut border_clr) in q_btn_unit.iter_mut() {
        match interaction {
            Interaction::None => border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND,
            Interaction::Pressed => {
                border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND_HOVER;
                if input.just_pressed(MouseButton::Left) {
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
    mut cursor_state: ResMut<CursorState>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if q_placeholder.is_empty() {
        return;
    }

    if input.just_pressed(MouseButton::Right) {
        for placeholder_ent in q_placeholder.iter() {
            *cursor_state = CursorState::Standard;
            cmds.entity(placeholder_ent).despawn_recursive();
        }
    }
}

fn select_structure(
    trigger: Trigger<BuildStructureSelectEv>,
    q_placeholder: Query<Entity, With<BuildStructurePlaceholder>>,
    dbg: Res<DbgOptions>,
    mut cursor_state: ResMut<CursorState>,
    mut cmds: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    dbg.print("Select Structure");

    let placeholder = trigger.event().0.clone();

    for placeholder_ent in q_placeholder.iter() {
        cmds.entity(placeholder_ent).despawn_recursive();
    }

    let placeholder_properties = placeholder.build(meshes, materials);

    *cursor_state = CursorState::Build;
    cmds.trigger(DeselectAllEv);
    cmds.spawn((placeholder_properties, BuildStructurePlaceholder));
}

fn build_structure(
    mut cmds: Commands,
    q_placeholder: Query<(Entity, &pf_comps::RtsObjSize), With<BuildStructurePlaceholder>>,
    dbg: Res<DbgOptions>,
    input: Res<ButtonInput<MouseButton>>,
    mut cursor_state: ResMut<CursorState>,
) {
    if *cursor_state != CursorState::Build {
        return;
    }

    if input.just_pressed(MouseButton::Left) {
        for (placeholder_ent, size) in q_placeholder.iter() {
            *cursor_state = CursorState::Standard;
            cmds.entity(placeholder_ent)
                .remove::<BuildStructurePlaceholder>();
            cmds.entity(placeholder_ent).insert(pf_comps::RtsObj);
            cmds.entity(placeholder_ent).insert(Collider::cuboid(
                size.0.x / 2.0,
                size.0.y / 2.0,
                size.0.z / 2.0,
            ));
        }

        dbg.print("Build Structure");
    }
}

fn sync_placeholder(
    mut q_placeholder: Query<
        (&mut Transform, &pf_comps::RtsObjSize),
        With<BuildStructurePlaceholder>,
    >,
    cam_q: Query<(&Camera, &GlobalTransform), With<pf_comps::GameCamera>>,
    map_base_q: Query<&GlobalTransform, With<pf_comps::MapBase>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok((mut transform, size)) = q_placeholder.get_single_mut() else {
        return;
    };

    let (cam, cam_trans) = cam_q.single();

    let Some(viewport_cursor) = q_window.single().cursor_position() else {
        return;
    };

    let coords = utils::get_world_coords(map_base_q.single(), &cam_trans, &cam, viewport_cursor);

    if let Some(coords) = coords {
        transform.translation = coords;
        transform.translation.y = size.0.y / 2.0;
    }
}
