use bevy::prelude::*;

use crate::resources::MyAssets;

pub struct CmdCenterUiPlugin;

impl Plugin for CmdCenterUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, command_center_ui);
    }
}

#[derive(Component)]
struct RootCtr;

#[derive(Component)]
struct MiniMapCtr;

#[derive(Component)]
struct IconsCtr;

#[derive(Component)]
struct BuildBtnsCtr;

fn command_center_ui(mut cmds: Commands, my_assets: Res<MyAssets>) {
    let root_ctr = (
        RootCtr,
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
        Name::new("Command Center Ctr"),
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

    let build_btns_ctr = (
        BuildBtnsCtr,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(60.0),
            ..default()
        },
        Name::new("Build Btns Ctr"),
    );

    let build_column = |margin_l: f32, margin_r: f32| -> (BackgroundColor, Node, Name) {
        (
            BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
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

    cmds.spawn(root_ctr).with_children(|parent| {
        parent.spawn(mini_map_ctr);
        parent.spawn(icons_ctr).with_children(|parent| {
            parent.spawn(icon(my_assets.cmd_cntr_structures.clone()));
            parent.spawn(icon(my_assets.cmd_cntr_units.clone()));
        });

        parent.spawn(build_btns_ctr).with_children(|parent| {
            parent.spawn(build_column(5.0, 2.5));
            parent.spawn(build_column(2.5, 5.0));
        });
    });
}
