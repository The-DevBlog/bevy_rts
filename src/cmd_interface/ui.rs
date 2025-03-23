use accesskit::{Node as Accessible, Role};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::focus::HoverMap;
use bevy::{a11y::AccessibilityNode, prelude::*};
use strum::IntoEnumIterator;

use super::{build_actions::CLR_STRUCTURE_BUILD_ACTIONS, components::*};
use crate::components::structures::Structure;
use crate::components::units::UnitType;
use crate::{bank::Bank, resources::MyAssets};

pub struct UiPlugin;

const CLR_BASE: Color = Color::srgb(0.29, 0.29, 0.3);
const CLR_BORDER_1: Color = Color::srgb(0.89, 0.89, 0.89);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, command_center_ui)
            .add_systems(Update, (update_minimap_aspect, update_scroll_position));
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
struct Cost;

#[derive(Component)]
pub struct CostCtr;

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
    let root_ctr = (
        CmdInterfaceCtr,
        Button,
        ImageNode::new(my_assets.imgs.cmd_intrfce_background.clone()),
        Node {
            padding: UiRect::left(Val::Percent(0.75)),
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            height: Val::Percent(100.0),
            width: Val::Percent(15.0),
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
            margin: UiRect::bottom(Val::Px(41.0)),
            top: Val::Px(22.1),
            left: Val::Percent(2.0),
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
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        Node {
            padding: UiRect::top(Val::Px(5.0)),
            margin: UiRect::new(Val::Px(12.0), Val::Px(10.0), Val::ZERO, Val::ZERO),
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

    let structure_opt_ctr = |structure: Structure,
                             assets: &Res<MyAssets>|
     -> (
        OptCtr,
        Button,
        BorderColor,
        ImageNode,
        Node,
        Structure,
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

    let unit_opt_ctr = |unit: UnitType,
                        assets: &Res<MyAssets>|
     -> (OptCtr, Button, BorderColor, ImageNode, Node, UnitCtr, Name) {
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
    };

    let build_opt_txt = |txt: &str| -> (
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
    };

    let cost = |cost: i32| -> (Cost, Text, Name) {
        (Cost, Text::new(format!("${}", cost)), Name::new("Cost"))
    };

    let cost_ctr = |cost: i32| -> (CostCtr, BackgroundColor, BorderColor, Node, Name) {
        let cost_str = cost.to_string();
        let offset = match cost_str.len() {
            4 => -87.5, // 4-digit cost
            _ => -75.5, // default for other cases
        };

        (
            CostCtr,
            BackgroundColor(CLR_BASE),
            BorderColor(CLR_BORDER_1),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(50.0),
                border: UiRect::new(Val::Px(5.0), Val::ZERO, Val::Px(5.0), Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                left: Val::Px(offset),
                ..default()
            },
            Name::new("Cost Ctr"),
        )
    };

    let spawn_structure_btn =
        |parent: &mut ChildBuilder, structure: Structure, assets: &Res<MyAssets>| {
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

                    p.spawn(cost_ctr(structure.cost()))
                        .with_child(cost(structure.cost()));
                });
        };

    let spawn_unit_btn = |parent: &mut ChildBuilder, unit: UnitType, assets: &Res<MyAssets>| {
        parent
            .spawn(unit_opt_ctr(unit, assets))
            .insert(PickingBehavior {
                should_block_lower: false,
                ..default()
            })
            .with_children(|p| {
                p.spawn(build_opt_txt(unit.to_string()))
                    .insert(PickingBehavior {
                        should_block_lower: false,
                        ..default()
                    });

                p.spawn(cost_ctr(unit.cost())).with_child(cost(unit.cost()));
            });
    };

    // Root Container
    cmds.spawn(root_ctr).with_children(|p| {
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
                    for structure in Structure::iter() {
                        spawn_structure_btn(parent, structure, &my_assets);
                    }
                    for structure in Structure::iter() {
                        spawn_structure_btn(parent, structure, &my_assets);
                    }
                });

                // Units Column
                p.spawn(build_column(5.0, 2.5)).with_children(|parent| {
                    for unit in UnitType::iter() {
                        spawn_unit_btn(parent, unit, &my_assets);
                    }
                });

                // p.spawn(build_column(2.5, 5.0)).with_children(|p| {
                // for unit in Unit::iter() {
                // p.spawn((unit_opt_ctr(unit, &my_assets), UnitCtr));
                // .with_child(build_opt_txt(unit_str));
                // }

                // p.spawn((unit_opt_ctr(), UnitCtr))
                //     .with_child(build_opt_txt("Unit 1"));
                // p.spawn((unit_opt_ctr(), UnitCtr))
                //     .with_child(build_opt_txt("Unit 2"));
                // p.spawn((unit_opt_ctr(), UnitCtr))
                //     .with_child(build_opt_txt("Unit 3"));
                // p.spawn((unit_opt_ctr(), UnitCtr))
                //     .with_child(build_opt_txt("Unit 4"));
                // p.spawn((unit_opt_ctr(), UnitCtr))
                //     .with_child(build_opt_txt("Unit 5"));
                // });
            });
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
