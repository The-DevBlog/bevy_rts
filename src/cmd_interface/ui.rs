use bevy::prelude::*;

use super::components::*;
use crate::resources::MyAssets;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, command_center_ui);
    }
}

fn command_center_ui(mut cmds: Commands, my_assets: Res<MyAssets>) {
    let root_ctr = (
        CmdInterfaceCtr,
        Button,
        Node {
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            height: Val::Percent(100.0),
            width: Val::Percent(18.0),
            min_width: Val::Px(225.0),
            max_width: Val::Px(500.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.29, 0.29, 0.3)),
        Name::new("Command Interface Ctr"),
    );

    let mini_map_ctr = (
        MiniMapCtr,
        Node {
            margin: UiRect::all(Val::Px(5.0)),
            height: Val::Percent(30.0),
            ..default()
        },
        Text::new("Mini Map"),
        TextLayout::new_with_justify(JustifyText::Center),
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        Name::new("Mini Map Ctr"),
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
            width: Val::Percent(100.0),
            height: Val::Percent(60.0),
            padding: UiRect::top(Val::Px(5.0)),
            margin: UiRect::new(Val::Px(5.0), Val::Px(5.0), Val::ZERO, Val::ZERO),
            ..default()
        },
        Name::new("Build Columns Ctr"),
    );

    let build_column = |margin_l: f32, margin_r: f32| -> (Node, Name) {
        (
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                margin: UiRect::new(Val::Px(margin_l), Val::Px(margin_r), Val::ZERO, Val::ZERO),
                ..default()
            },
            Name::new("Build Column"),
        )
    };

    let structure_opt_ctr = |structure: StructureType,
                             assets: &Res<MyAssets>|
     -> (Button, BorderColor, ImageNode, Node, Structure, Name) {
        (
            Button,
            BorderColor(Color::srgb(0.8, 0.8, 0.8)),
            ImageNode {
                image: structure.img(assets),
                ..default()
            },
            Node {
                height: Val::Percent(20.0),
                margin: UiRect::bottom(Val::Px(5.0)),
                border: UiRect::all(Val::Px(2.5)),
                ..default()
            },
            Structure(structure),
            Name::new("Build Option"),
        )
    };

    let unit_opt_ctr = || -> (Button, BorderColor, Node, Name) {
        (
            Button,
            BorderColor(Color::srgb(0.8, 0.8, 0.8)),
            Node {
                height: Val::Percent(20.0),
                margin: UiRect::bottom(Val::Px(5.0)),
                border: UiRect::all(Val::Px(2.5)),
                ..default()
            },
            Name::new("Build Option"),
        )
    };

    let build_opt = |txt: &str| -> (Node, Text, TextFont, TextLayout, Name) {
        (
            Node {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            Text::new(txt),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            Name::new("Build Option Ctr"),
        )
    };

    let spawn_structure_btn =
        |parent: &mut ChildBuilder, structure: StructureType, assets: &Res<MyAssets>| {
            parent
                .spawn(structure_opt_ctr(structure, assets))
                .with_child(build_opt(structure.to_string()));
        };

    // Root Container
    cmds.spawn(root_ctr).with_children(|p| {
        //  Mini Map
        p.spawn(mini_map_ctr);

        // Structure/Units Icons
        p.spawn(icons_ctr).with_children(|parent| {
            parent.spawn(icon(my_assets.images.cmd_intrfce_structures.clone()));
            parent.spawn(icon(my_assets.images.cmd_intrfce_units.clone()));
        });

        // Structure/Units Columns
        p.spawn(build_columns_ctr)
            .with_children(|p: &mut ChildBuilder<'_>| {
                // Structures Column
                p.spawn(build_column(5.0, 2.5)).with_children(|p| {
                    spawn_structure_btn(p, StructureType::Turret, &my_assets);
                    spawn_structure_btn(p, StructureType::Barracks, &my_assets);
                    spawn_structure_btn(p, StructureType::VehicleDepot, &my_assets);
                    spawn_structure_btn(p, StructureType::Black, &my_assets);
                    spawn_structure_btn(p, StructureType::White, &my_assets);
                });

                // Units Column
                p.spawn(build_column(2.5, 5.0)).with_children(|p| {
                    p.spawn((unit_opt_ctr(), Unit))
                        .with_child(build_opt("Unit 1"));
                    p.spawn((unit_opt_ctr(), Unit))
                        .with_child(build_opt("Unit 2"));
                    p.spawn((unit_opt_ctr(), Unit))
                        .with_child(build_opt("Unit 3"));
                    p.spawn((unit_opt_ctr(), Unit))
                        .with_child(build_opt("Unit 4"));
                    p.spawn((unit_opt_ctr(), Unit))
                        .with_child(build_opt("Unit 5"));
                });
            });
    });
}
