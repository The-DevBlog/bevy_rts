use bevy::prelude::*;

pub struct BaseBuildingPlugin;

impl Plugin for BaseBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, command_center_ui);
    }
}

#[derive(Component)]
struct RootCtr;

#[derive(Component)]
struct MiniMapCtr;

fn command_center_ui(mut cmds: Commands) {
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

    cmds.spawn(root_ctr).with_child(mini_map_ctr);
}
