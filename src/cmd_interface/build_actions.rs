use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::Collider;
use bevy_rts_camera::RtsCameraControls;

use super::components::*;
use super::events::*;
use crate::events::DeselectAllEv;
use crate::resources::CursorState;
use crate::resources::DbgOptions;
use crate::resources::GameCommands;
use crate::resources::MyAssets;
use crate::utils;
use bevy_rts_pathfinding::components::{self as pf_comps};

const CLR_BUILD_ACTIONS_BACKGROUND: Color = Color::srgb(0.07, 0.07, 0.07);
const CLR_BUILD_ACTIONS_BACKGROUND_HOVER: Color = Color::srgb(0.2, 0.2, 0.2);
pub const CLR_STRUCTURE_BUILD_ACTIONS: Color = Color::srgb(0.87, 0.87, 1.0);
const CLR_STRUCTURE_BUILD_ACTIONS_HVR: Color = Color::srgb(1.0, 1.0, 1.0);

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
    mut q_btn_bldg: Query<(&Interaction, &mut ImageNode, &Structure), With<Structure>>,
) {
    for (interaction, mut img, structure) in q_btn_bldg.iter_mut() {
        match interaction {
            Interaction::None => img.color = CLR_STRUCTURE_BUILD_ACTIONS,
            Interaction::Pressed => {
                img.color = CLR_STRUCTURE_BUILD_ACTIONS_HVR;
                if input.just_pressed(MouseButton::Left) {
                    cmds.trigger(BuildStructureSelectEv(structure.clone()));
                }
            }
            Interaction::Hovered => img.color = CLR_STRUCTURE_BUILD_ACTIONS_HVR,
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
            Interaction::Hovered => border_clr.0 = CLR_BUILD_ACTIONS_BACKGROUND_HOVER,
        }
    }
}

fn cancel_build_structure(
    q_placeholder: Query<Entity, With<BuildStructurePlaceholder>>,
    mut cmds: Commands,
    mut cursor_state: ResMut<CursorState>,
    input: Res<ButtonInput<MouseButton>>,
    mut q_cam_ctrls: Query<&mut RtsCameraControls, With<pf_comps::GameCamera>>,
) {
    if q_placeholder.is_empty() {
        if let Ok(mut ctrls) = q_cam_ctrls.get_single_mut() {
            ctrls.zoom_sensitivity = 0.2;
        };

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
    my_assets: Res<MyAssets>,
) {
    dbg.print("Select Structure");

    let placeholder = trigger.event().0.clone();

    for placeholder_ent in q_placeholder.iter() {
        cmds.entity(placeholder_ent).despawn_recursive();
    }

    let placeholder_properties = placeholder.build(my_assets);
    let transform = Transform::from_xyz(100000.0, 0.0, 0.0); // avoid bug flicker

    *cursor_state = CursorState::Build;
    cmds.trigger(DeselectAllEv);
    cmds.spawn((
        placeholder_properties,
        transform,
        placeholder,
        BuildStructurePlaceholder,
    ));
}

fn build_structure(
    mut cmds: Commands,
    mut q_placeholder: Query<
        (Entity, &Structure, &mut SceneRoot, &pf_comps::RtsObjSize),
        With<BuildStructurePlaceholder>,
    >,
    dbg: Res<DbgOptions>,
    input: Res<ButtonInput<MouseButton>>,
    mut cursor_state: ResMut<CursorState>,
    my_assets: Res<MyAssets>,
) {
    if *cursor_state != CursorState::Build {
        return;
    }

    if input.just_pressed(MouseButton::Left) {
        let Ok((placeholder_ent, structure, mut scene, size)) = q_placeholder.get_single_mut()
        else {
            return;
        };

        *cursor_state = CursorState::Standard;
        structure.place(&my_assets, &mut scene);
        cmds.entity(placeholder_ent)
            .remove::<BuildStructurePlaceholder>();
        cmds.entity(placeholder_ent).insert(pf_comps::RtsObj);
        cmds.entity(placeholder_ent).insert(Collider::cuboid(
            size.0.x / 2.0,
            size.0.y / 2.0,
            size.0.z / 2.0,
        ));

        dbg.print("Build Structure");
    }
}

fn sync_placeholder(
    mut q_placeholder: Query<
        (&mut Transform, &pf_comps::RtsObjSize),
        With<BuildStructurePlaceholder>,
    >,
    mut cam_q: Query<
        (&Camera, &GlobalTransform, &mut RtsCameraControls),
        With<pf_comps::GameCamera>,
    >,
    map_base_q: Query<&GlobalTransform, With<pf_comps::MapBase>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let Ok((mut transform, size)) = q_placeholder.get_single_mut() else {
        return;
    };

    const ROTATION_SPEED: f32 = 0.3;
    for event in mouse_wheel_events.read() {
        transform.rotate_y(event.y * ROTATION_SPEED);
    }

    let (cam, cam_trans, mut rts_cam_ctrls) = cam_q.single_mut();
    rts_cam_ctrls.zoom_sensitivity = 0.0;

    let Some(viewport_cursor) = q_window.single().cursor_position() else {
        return;
    };

    let coords = utils::get_world_coords(map_base_q.single(), &cam_trans, &cam, viewport_cursor);
    if let Some(coords) = coords {
        transform.translation = coords;
        transform.translation.y = size.0.y / 2.0;
    }
}
