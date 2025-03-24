use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::RigidBody;
use bevy_rapier3d::prelude::*;

use super::components::*;
use super::events::*;
use super::resources::InfoContainerData;
use crate::bank::AdjustFundsEv;
use crate::bank::Bank;
use crate::components::structures::*;
use crate::events::DeselectAllEv;
use crate::resources::*;
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
                cancel_build_structure,
                cmd_interface_interaction,
                reset_info_ctr_hvr_state,
                build_structure_btn_interaction.after(reset_info_ctr_hvr_state),
                build_unit_btn_interaction.after(reset_info_ctr_hvr_state),
                sync_placeholder,
                validate_structure_placement,
                place_structure.after(validate_structure_placement),
                toggle_info_ctr,
            ),
        )
        .add_observer(select_structure);
    }
}

fn cmd_interface_interaction(
    mut game_cmds: ResMut<GameCommands>,
    q_p: Query<&Interaction, With<CmdInterfaceCtr>>,
    q_c: Query<&Interaction, Or<(With<Structure>, With<UnitCtr>)>>,
) {
    let hvr_parent = q_p.iter().any(|intrct| *intrct == Interaction::Hovered);
    let hvr_child = q_c.iter().any(|intrct| *intrct == Interaction::Hovered);
    let click_parent = q_p.iter().any(|intrct| *intrct == Interaction::Pressed);
    let click_child = q_c.iter().any(|intrct| *intrct == Interaction::Pressed);

    game_cmds.hvr_cmd_interface = hvr_parent || hvr_child || click_parent || click_child;
}

fn build_structure_btn_interaction(
    mut cmds: Commands,
    mut q_btn_bldg: Query<(&Interaction, &mut ImageNode, &Structure), With<Structure>>,
    bank: Res<Bank>,
    dbg: Res<DbgOptions>,
    mut info_ctr_data: ResMut<InfoContainerData>,
) {
    for (interaction, mut img, structure) in q_btn_bldg.iter_mut() {
        let name = structure.to_string();
        let cost = structure.cost();

        match interaction {
            Interaction::None => {
                img.color = CLR_STRUCTURE_BUILD_ACTIONS;
            }
            Interaction::Pressed => {
                info_ctr_data.active = true;
                img.color = CLR_STRUCTURE_BUILD_ACTIONS_HVR;
                if bank.funds >= structure.cost() {
                    cmds.trigger(BuildStructureSelectEv(structure.clone()));
                } else {
                    dbg.print("Not enough funds");
                }
            }
            Interaction::Hovered => {
                info_ctr_data.active = true;
                info_ctr_data.name = name.clone();
                info_ctr_data.cost = cost;
                img.color = CLR_STRUCTURE_BUILD_ACTIONS_HVR;
            }
        }
    }
}

fn build_unit_btn_interaction(
    mut q_btn_unit: Query<(&Interaction, &mut ImageNode, &UnitCtr), With<UnitCtr>>,
    bank: Res<Bank>,
    dbg: Res<DbgOptions>,
    mut info_ctr_data: ResMut<InfoContainerData>,
) {
    for (interaction, mut img, unit_ctr) in q_btn_unit.iter_mut() {
        let cost = unit_ctr.0.cost();
        let name = unit_ctr.0.to_string();

        match interaction {
            Interaction::None => {
                img.color = CLR_STRUCTURE_BUILD_ACTIONS;
            }
            Interaction::Pressed => {
                img.color = CLR_STRUCTURE_BUILD_ACTIONS_HVR;
            }
            Interaction::Hovered => {
                info_ctr_data.active = true;
                info_ctr_data.name = name.clone();
                info_ctr_data.cost = cost;
                img.color = CLR_STRUCTURE_BUILD_ACTIONS_HVR;
            }
        }
    }
}

fn cancel_build_structure(
    q_placeholder: Query<Entity, With<StructurePlaceholder>>,
    mut cmds: Commands,
    mut cursor_state: ResMut<CursorState>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Right) {
        for placeholder_ent in q_placeholder.iter() {
            *cursor_state = CursorState::Standard;
            cmds.entity(placeholder_ent).despawn_recursive();
        }
    }
}

fn select_structure(
    trigger: Trigger<BuildStructureSelectEv>,
    q_placeholder: Query<Entity, With<StructurePlaceholder>>,
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

    let placeholder_properties = placeholder.build_placeholder(my_assets);
    let transform = Transform::from_xyz(100000.0, 0.0, 0.0); // avoid bug flicker

    *cursor_state = CursorState::Build;
    cmds.trigger(DeselectAllEv);
    cmds.spawn((placeholder_properties, transform, placeholder));
}

fn place_structure(
    mut cmds: Commands,
    mut q_placeholder: Query<
        (
            Entity,
            &StructurePlaceholder,
            &Structure,
            &mut RigidBody,
            &mut SceneRoot,
            &pf_comps::RtsObjSize,
        ),
        With<StructurePlaceholder>,
    >,
    dbg: Res<DbgOptions>,
    input: Res<ButtonInput<MouseButton>>,
    mut cursor_state: ResMut<CursorState>,
    my_assets: Res<MyAssets>,
) {
    if *cursor_state != CursorState::Build {
        return;
    }

    let Ok((placeholder_ent, placeholder, structure, mut rb, mut scene, _size)) =
        q_placeholder.get_single_mut()
    else {
        return;
    };

    if input.just_pressed(MouseButton::Left) && placeholder.is_valid {
        *cursor_state = CursorState::Standard;
        structure.place(placeholder_ent, &my_assets, &mut scene, &mut rb, &mut cmds);

        // Adjust bank
        cmds.trigger(AdjustFundsEv(-structure.cost()));

        // place structure audio
        let audio = AudioPlayer::new(my_assets.audio.place_structure.clone());
        cmds.spawn(audio);

        dbg.print("Build Structure");
    }
}

fn sync_placeholder(
    mut q_placeholder: Query<(&mut Transform, &pf_comps::RtsObjSize), With<StructurePlaceholder>>,
    mut cam_q: Query<(&Camera, &GlobalTransform), With<pf_comps::GameCamera>>,
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

    let (cam, cam_trans) = cam_q.single_mut();

    let Some(viewport_cursor) = q_window.single().cursor_position() else {
        return;
    };

    let coords = utils::get_world_coords(map_base_q.single(), &cam_trans, &cam, viewport_cursor);
    if let Some(coords) = coords {
        transform.translation = coords;
        transform.translation.y = size.0.y / 2.0;
    }
}

fn validate_structure_placement(
    q_rapier: Query<&RapierContext, With<DefaultRapierContext>>,
    mut q_placeholder: Query<(Entity, &mut StructurePlaceholder, &mut SceneRoot)>,
    q_collider: Query<&Collider, With<pf_comps::MapBase>>,
    my_assets: Res<MyAssets>,
) {
    let Ok((placeholder_ent, mut placeholder, mut scene)) = q_placeholder.get_single_mut() else {
        return;
    };

    let Ok(rapier_ctx) = q_rapier.get_single() else {
        return;
    };

    let mut is_colliding = false;
    for (ent_1, ent_2, intersect) in rapier_ctx.intersection_pairs_with(placeholder_ent) {
        // exclude any collisions with the map base
        if q_collider.get(ent_1).is_ok() || q_collider.get(ent_2).is_ok() {
            continue;
        }

        is_colliding = intersect;
    }

    if is_colliding {
        placeholder.is_valid = false;
        placeholder
            .structure
            .invalid_placement(&my_assets, &mut scene);
    } else {
        placeholder.is_valid = true;
        placeholder
            .structure
            .valid_placement(&my_assets, &mut scene);
    }
}

fn reset_info_ctr_hvr_state(mut info_ctr_data: ResMut<InfoContainerData>) {
    info_ctr_data.active = false;
}

fn toggle_info_ctr(
    mut q_info_ctr: Query<&mut Visibility, With<InfoCtr>>,
    info_ctr_data: Res<InfoContainerData>,
    mut q_cost: Query<&mut Text, With<InfoCtrCost>>,
    mut q_name: Query<&mut Text, (With<InfoCtrName>, Without<InfoCtrCost>)>,
) {
    let Ok(mut info_ctr_vis) = q_info_ctr.get_single_mut() else {
        return;
    };

    if info_ctr_data.active {
        *info_ctr_vis = Visibility::Visible;
    } else {
        *info_ctr_vis = Visibility::Hidden;
    }

    let Ok(mut cost) = q_cost.get_single_mut() else {
        return;
    };

    let Ok(mut name) = q_name.get_single_mut() else {
        return;
    };

    name.0 = info_ctr_data.name.to_string();
    cost.0 = format!("${}", info_ctr_data.cost);
}
