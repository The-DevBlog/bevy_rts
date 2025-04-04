use accesskit::{Node as Accessible, Role};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::focus::HoverMap;
use bevy::{a11y::AccessibilityNode, prelude::*};
use strum::IntoEnumIterator;

use super::{build_actions::CLR_STRUCTURE_BUILD_ACTIONS, components::*};
use crate::components::structures::StructureType;
use crate::components::units::UnitType;
use crate::resources::units::UnlockedUnits;
use crate::{bank::Bank, resources::MyAssets};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, command_center_ui).add_systems(
            Update,
            (
                update_minimap_aspect,
                update_scroll_position,
                spawn_unit_ctrs.run_if(resource_changed::<UnlockedUnits>),
            ),
        );
    }
}

#[derive(Component)]
struct OptCtr;

#[derive(Component)]
struct MiniMapCtr;

#[derive(Component)]
pub struct BankCtr;

#[derive(Component)]
struct BuildColumnsCtr;

#[derive(Component)]
struct IconsCtr;

#[derive(Component)]
struct TankGen1Ctr;

#[derive(Component)]
struct TankGen2Ctr;

#[derive(Component)]
struct RiflemanCtr;

fn update_minimap_aspect(mut q_mini_map: Query<(&mut Node, &ComputedNode), With<MiniMapCtr>>) {
    if let Ok((mut mini_map, computed_node)) = q_mini_map.get_single_mut() {
        let width = computed_node.size().x;

        // first frame is 0.0 for some reason
        if width == 0.0 {
            return;
        }

        mini_map.height = Val::Px(width);
    }
}

fn command_center_ui(mut cmds: Commands, my_assets: Res<MyAssets>, bank: Res<Bank>) {
    let info_ctr = (
        InfoCtr,
        ImageNode::new(my_assets.imgs.info_ctr.clone()),
        Node {
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Px(10.0)),
            align_self: AlignSelf::FlexStart,
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            top: Val::Percent(50.0),
            ..default()
        },
        ZIndex(105),
        Name::new("Info Ctr"),
    );

    fn create_ctr<T>(ctr: T, name: &str) -> (T, Node, Name) {
        (
            ctr,
            Node {
                padding: UiRect::new(Val::Px(5.0), Val::ZERO, Val::Px(5.0), Val::Px(5.0)),
                ..default()
            },
            Name::new(name.to_string()),
        )
    }

    let info_ctr_icon = |img: Handle<Image>, name: String| -> (ImageNode, Node, Name) {
        (
            ImageNode::new(img),
            Node {
                margin: UiRect::right(Val::Px(7.5)),
                ..default()
            },
            Name::new(name),
        )
    };

    // Info Ctr Data
    let name = (InfoCtrName, Text::new("Building Name"), Name::new("Name"));
    let cost = (InfoCtrCost, Text::new("$1000"), Name::new("Cost"));
    let speed_txt = (InfoCtrSpeedTxt, Text::new(""), Name::new("Speed"));
    let dmg_txt = (InfoCtrDmgTxt, Text::new(""), Name::new("Dmg Txt"));
    let hp_txt = (InfoCtrHpTxt, Text::new(""), Name::new("HP Txt"));
    let build_time_txt = (
        InfoCtrBuildTimeTxt,
        Text::new(""),
        Name::new("Build Time Txt"),
    );

    let cmd_interface_ctr = (
        CmdInterfaceCtr,
        Button,
        ImageNode::new(my_assets.imgs.cmd_intrfce_background.clone()),
        Node {
            margin: UiRect::left(Val::Auto),
            flex_direction: FlexDirection::Column,
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            max_width: Val::Px(394.0),
            min_width: Val::Px(200.0),
            ..default()
        },
        Name::new("Command Interface Ctr"),
    );

    let mini_map_ctr = (
        MiniMapCtr,
        Node {
            min_height: Val::Percent(25.0),
            max_height: Val::Px(341.0),
            max_width: Val::Px(341.0),
            width: Val::Percent(100.0),
            margin: UiRect::bottom(Val::Px(41.0)),
            top: Val::Px(22.1),
            // left: Val::Percent(2.0),
            ..default()
        },
        Text::new("Mini Map"),
        TextLayout::new_with_justify(JustifyText::Center),
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        Name::new("Mini Map Ctr"),
    );

    let bank_ctr = (
        BankCtr,
        Node {
            margin: UiRect::bottom(Val::Percent(2.8)),
            ..default()
        },
        Text::new(format!("${}", bank.funds.to_string())),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Name::new("Bank"),
    );

    let icons_ctr = (
        IconsCtr,
        Node {
            margin: UiRect::all(Val::Px(5.0)),
            width: Val::Percent(90.0),
            // height: Val::Px(61.0),
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        Name::new("Icons Ctr"),
    );

    let icon = |img: Handle<Image>| -> (ImageNode, Node, Name) {
        (
            ImageNode::new(img),
            Node {
                width: Val::Percent(25.0),
                ..default()
            },
            Name::new("Icon"),
        )
    };

    let build_columns_ctr = (
        BuildColumnsCtr,
        // BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        BackgroundColor(Color::BLACK),
        Node {
            padding: UiRect::top(Val::Px(5.0)),
            // margin: UiRect::new(Val::Auto, Val::Auto, Val::ZERO, Val::ZERO),
            min_width: Val::Px(246.0),
            max_width: Val::Px(358.0),
            width: Val::Percent(100.0),
            // max_width: Val::Px(341.0),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        Name::new("Build Columns Ctr"),
    );

    let build_column = |margin_l: f32, margin_r: f32| -> (Node, Name) {
        (
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                margin: UiRect::new(Val::Px(margin_l), Val::Px(margin_r), Val::ZERO, Val::ZERO),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            Name::new("Build Column"),
        )
    };

    let structure_opt_ctr = |structure: StructureType,
                             assets: &Res<MyAssets>|
     -> (
        OptCtr,
        Button,
        BorderColor,
        ImageNode,
        Node,
        StructureType,
        Name,
    ) {
        (
            OptCtr,
            Button,
            BorderColor(Color::srgb(0.8, 0.8, 0.8)),
            ImageNode {
                image: structure.img(assets),
                color: CLR_STRUCTURE_BUILD_ACTIONS,
                ..default()
            },
            Node {
                width: Val::Percent(100.0),
                min_width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(5.0)),
                border: UiRect::all(Val::Px(2.5)),
                aspect_ratio: Some(1.0),
                ..default()
            },
            structure,
            Name::new("Structure Build Option"),
        )
    };

    let spawn_structure_btn =
        |parent: &mut ChildBuilder, structure: StructureType, assets: &Res<MyAssets>| {
            parent
                .spawn(structure_opt_ctr(structure, assets))
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(build_opt_txt(structure.to_string()))
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        });
                });
        };

    // Info Ctr
    cmds.spawn(info_ctr).with_children(|p| {
        p.spawn(name);
        p.spawn(cost);
        p.spawn(create_ctr(InfoCtrBuildTime, "Build Time Ctr"))
            .with_children(|p| {
                p.spawn(info_ctr_icon(
                    my_assets.imgs.info_ctr_build_time.clone(),
                    "Build Time Icon".to_string(),
                ));
                p.spawn(build_time_txt);
            });
        p.spawn(create_ctr(InfoCtrHp, "HP Ctr")).with_children(|p| {
            p.spawn(info_ctr_icon(
                my_assets.imgs.info_ctr_hp.clone(),
                "HP Icon".to_string(),
            ));
            p.spawn(hp_txt);
        });
        p.spawn(create_ctr(InfoCtrDmg, "Dmg Ctr"))
            .with_children(|p| {
                p.spawn(info_ctr_icon(
                    my_assets.imgs.info_ctr_dmg.clone(),
                    "Dmg Icon".to_string(),
                ));
                p.spawn(dmg_txt);
            });
        p.spawn(create_ctr(InfoCtrSpeed, "Speed Ctr"))
            .with_children(|p| {
                p.spawn(info_ctr_icon(
                    my_assets.imgs.info_ctr_speed.clone(),
                    "Speed Icon".to_string(),
                ));
                p.spawn(speed_txt);
            });
    });

    // Command Interface Ctr
    cmds.spawn(cmd_interface_ctr).with_children(|p| {
        //  Mini Map
        p.spawn(mini_map_ctr);

        // Bank
        p.spawn(bank_ctr);

        // Structure/Units Icons
        p.spawn(icons_ctr).with_children(|parent| {
            parent.spawn(icon(my_assets.imgs.cmd_intrfce_structures.clone()));
            parent.spawn(icon(my_assets.imgs.cmd_intrfce_units.clone()));
        });

        // Structure/Units Columns
        p.spawn(build_columns_ctr)
            .with_children(|p: &mut ChildBuilder<'_>| {
                // Structures Column
                p.spawn(build_column(5.0, 2.5)).with_children(|parent| {
                    for structure in StructureType::iter() {
                        spawn_structure_btn(parent, structure, &my_assets);
                    }
                    for structure in StructureType::iter() {
                        spawn_structure_btn(parent, structure, &my_assets);
                    }
                });

                // Units Column
                p.spawn((build_column(5.0, 2.5), UnitBuildColumn));
            });
    });
}

fn spawn_unit_ctrs(
    mut cmds: Commands,
    q_unit_build_column: Query<Entity, With<UnitBuildColumn>>,
    unlocked_units: Res<UnlockedUnits>,
    my_assets: Res<MyAssets>,
) {
    let Ok(unit_build_column) = q_unit_build_column.get_single() else {
        return;
    };

    cmds.entity(unit_build_column).despawn_descendants();

    // Now add the unit control buttons in the desired order.
    cmds.entity(unit_build_column).with_children(|parent| {
        if unlocked_units.rifleman {
            spawn_unit_btn(parent, UnitType::Rifleman, &my_assets, RiflemanCtr);
        }
        if unlocked_units.tank_gen1 {
            spawn_unit_btn(parent, UnitType::TankGen1, &my_assets, TankGen1Ctr);
        }
        if unlocked_units.tank_gen2 {
            spawn_unit_btn(parent, UnitType::TankGen2, &my_assets, TankGen2Ctr);
        }
    });
}

pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 25.0, mouse_wheel_event.y * 25.0),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

fn unit_opt_ctr(
    unit: UnitType,
    assets: &Res<MyAssets>,
) -> (OptCtr, Button, BorderColor, ImageNode, Node, UnitCtr, Name) {
    (
        OptCtr,
        Button,
        BorderColor(Color::srgb(0.8, 0.8, 0.8)),
        ImageNode::from(unit.img(assets)),
        Node {
            width: Val::Percent(100.0),
            min_width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Px(5.0)),
            border: UiRect::all(Val::Px(2.5)),
            aspect_ratio: Some(1.0),
            ..default()
        },
        UnitCtr(unit),
        Name::new("Unit Build Option"),
    )
}

fn spawn_unit_btn<T: Component>(
    parent: &mut ChildBuilder,
    unit: UnitType,
    assets: &Res<MyAssets>,
    comp: T,
) {
    parent
        .spawn(unit_opt_ctr(unit, assets))
        .insert(comp)
        .insert(PickingBehavior {
            should_block_lower: false,
            ..default()
        })
        .with_children(|p| {
            p.spawn(build_opt_txt(unit.name())).insert(PickingBehavior {
                should_block_lower: false,
                ..default()
            });
        });
}

fn build_opt_txt(
    txt: String,
) -> (
    Node,
    Text,
    TextFont,
    TextLayout,
    Label,
    AccessibilityNode,
    Name,
) {
    (
        Node {
            margin: UiRect::top(Val::Auto),
            ..default()
        },
        Text::new(txt),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Label,
        AccessibilityNode(Accessible::new(Role::ListItem)),
        Name::new("Build Option Txt"),
    )
}
