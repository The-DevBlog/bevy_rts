use bevy::prelude::*;

use super::components::*;
use super::events::*;
use super::resources::InfoContainerData;
use crate::asset_manager::models::MyModels;
use crate::bank::Bank;
use crate::components::structures::*;
use crate::events::DeselectAllEv;
use crate::resources::*;

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
                toggle_info_ctr,
            ),
        )
        .add_observer(select_structure);
    }
}

fn cmd_interface_interaction(
    mut game_cmds: ResMut<GameCommands>,
    q_p: Query<&Interaction, With<CmdInterfaceCtr>>,
    q_c: Query<&Interaction, Or<(With<StructureType>, With<UnitCtr>)>>,
) {
    let hvr_parent = q_p.iter().any(|intrct| *intrct == Interaction::Hovered);
    let hvr_child = q_c.iter().any(|intrct| *intrct == Interaction::Hovered);
    let click_parent = q_p.iter().any(|intrct| *intrct == Interaction::Pressed);
    let click_child = q_c.iter().any(|intrct| *intrct == Interaction::Pressed);

    game_cmds.hvr_cmd_interface = hvr_parent || hvr_child || click_parent || click_child;
}

fn build_structure_btn_interaction(
    mut cmds: Commands,
    mut q_btn_bldg: Query<(&Interaction, &mut ImageNode, &StructureType), With<StructureType>>,
    bank: Res<Bank>,
    dbg: Res<DbgOptions>,
    mut info_ctr_data: ResMut<InfoContainerData>,
) {
    for (interaction, mut img, structure) in q_btn_bldg.iter_mut() {
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
                info_ctr_data.name = structure.to_string();
                info_ctr_data.cost = structure.cost();
                info_ctr_data.build_time = structure.build_time();
                info_ctr_data.hp = None;
                info_ctr_data.dmg = None;
                info_ctr_data.speed = None;
                img.color = CLR_STRUCTURE_BUILD_ACTIONS_HVR;
            }
        }
    }
}

fn build_unit_btn_interaction(
    mut cmds: Commands,
    mut q_btn_unit: Query<(&Interaction, &mut ImageNode, &UnitCtr), With<UnitCtr>>,
    bank: Res<Bank>,
    dbg: Res<DbgOptions>,
    mut info_ctr_data: ResMut<InfoContainerData>,
    input: Res<ButtonInput<MouseButton>>,
) {
    for (interaction, mut img, unit_ctr) in q_btn_unit.iter_mut() {
        match interaction {
            Interaction::None => {
                img.color = CLR_STRUCTURE_BUILD_ACTIONS;
            }
            Interaction::Pressed => {
                info_ctr_data.active = true;
                img.color = CLR_STRUCTURE_BUILD_ACTIONS_HVR;

                if input.just_pressed(MouseButton::Left) {
                    cmds.trigger(BuildUnitEv(unit_ctr.0));
                }
            }
            Interaction::Hovered => {
                info_ctr_data.active = true;
                // TODO: Make this a method that will fill everything in?
                info_ctr_data.name = unit_ctr.0.name();
                info_ctr_data.cost = unit_ctr.0.cost();
                info_ctr_data.build_time = unit_ctr.0.build_time();
                info_ctr_data.hp = Some(unit_ctr.0.hp());
                info_ctr_data.dmg = Some(unit_ctr.0.dmg());
                info_ctr_data.speed = Some(unit_ctr.0.speed());
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
    my_models: Res<MyModels>,
) {
    dbg.print("Select Structure");

    let placeholder = trigger.event().0.clone();

    for placeholder_ent in q_placeholder.iter() {
        cmds.entity(placeholder_ent).despawn_recursive();
    }

    let placeholder_properties = placeholder.build_placeholder(my_models);
    let transform = Transform::from_xyz(100000.0, 0.0, 0.0); // avoid bug flicker

    *cursor_state = CursorState::Build;
    cmds.trigger(DeselectAllEv);
    cmds.spawn((placeholder_properties, transform, placeholder));
}

fn reset_info_ctr_hvr_state(mut info_ctr_data: ResMut<InfoContainerData>) {
    info_ctr_data.active = false;
}

fn toggle_info_ctr(
    q_cmd_interface: Query<&ComputedNode, With<CmdInterfaceCtr>>,
    // mut q_info_ctr: Query<(&mut Visibility, &mut Node), With<InfoCtr>>,
    info_ctr_data: Res<InfoContainerData>,
    mut set: ParamSet<(
        Query<&mut Text, With<InfoCtrDmgTxt>>,
        Query<&mut Text, With<InfoCtrSpeedTxt>>,
        Query<&mut Text, With<InfoCtrHpTxt>>,
        Query<&mut Text, With<InfoCtrBuildTimeTxt>>,
        Query<&mut Text, With<InfoCtrName>>,
        Query<&mut Text, With<InfoCtrCost>>,
    )>,
    mut ctr_set: ParamSet<(
        Query<&mut Node, With<InfoCtrDmg>>,
        Query<&mut Node, With<InfoCtrSpeed>>,
        Query<&mut Node, With<InfoCtrHp>>,
        Query<(&mut Node, &mut Visibility), With<InfoCtr>>,
    )>,
) {
    let Ok(cmd_interface_node) = q_cmd_interface.get_single() else {
        return;
    };

    let cmd_interface_width = cmd_interface_node.size().x;
    let info_ctr_x = cmd_interface_width + 10.0; // +10 for added margin

    if let Ok((mut info_ctr_node, mut info_ctr_vis)) = ctr_set.p3().get_single_mut() {
        if info_ctr_data.active {
            *info_ctr_vis = Visibility::Visible;
            info_ctr_node.right = Val::Px(info_ctr_x);
        } else {
            *info_ctr_vis = Visibility::Hidden;
        }
    }

    // Name
    if let Ok(mut name) = set.p4().get_single_mut() {
        name.0 = info_ctr_data.name.to_string();
    };

    // Cost
    if let Ok(mut cost) = set.p5().get_single_mut() {
        cost.0 = format!("${}", info_ctr_data.cost);
    };

    // Build Time
    if let Ok(mut build_time_txt) = set.p3().get_single_mut() {
        build_time_txt.0 = info_ctr_data.build_time.to_string();
    }

    // Damage
    if let Ok(mut dmg_txt) = set.p0().get_single_mut() {
        if let Ok(mut dmg_ctr) = ctr_set.p0().get_single_mut() {
            if let Some(dmg) = info_ctr_data.dmg {
                dmg_ctr.display = Display::Flex;
                dmg_txt.0 = dmg.to_string();
            } else {
                dmg_ctr.display = Display::None;
            }
        }
    }

    // Speed
    if let Ok(mut speed_txt) = set.p1().get_single_mut() {
        if let Ok(mut speed_ctr) = ctr_set.p1().get_single_mut() {
            if let Some(speed) = info_ctr_data.speed {
                speed_ctr.display = Display::Flex;
                speed_txt.0 = speed.to_string();
            } else {
                speed_ctr.display = Display::None;
            }
        }
    }

    // HP
    if let Ok(mut hp_txt) = set.p2().get_single_mut() {
        if let Ok(mut hp_ctr) = ctr_set.p2().get_single_mut() {
            if let Some(hp) = info_ctr_data.hp {
                hp_ctr.display = Display::Flex;
                hp_txt.0 = hp.to_string();
            } else {
                hp_ctr.display = Display::None;
            }
        }
    }
}
