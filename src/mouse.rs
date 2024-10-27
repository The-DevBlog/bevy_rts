use std::ptr::null;

use bevy::color::palettes::css::{DARK_GRAY, RED};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext};
use bevy_rts_camera::RtsCamera;

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::tank::set_unit_destination;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_selection_box)
            .add_systems(
                Update,
                (
                    set_mouse_coords,
                    handle_input,
                    set_drag_select,
                    draw_drag_select_box,
                    single_select,
                    set_selected,
                    deselect_all,
                )
                    .chain()
                    .after(set_unit_destination),
            )
            .observe(handle_drag_select)
            .observe(set_starting_point_drag_select_coords)
            .observe(set_drag_select_coords)
            .observe(clear_drag_select_coords);
    }
}

fn set_drag_select(box_coords: Res<SelectBox>, mut game_cmds: ResMut<GameCommands>) {
    let drag_threshold = 2.5;
    // let world = box_coords.coords_world.clone();
    // let width_z = (world.upper_1.z - world.upper_2.z).abs();
    // let width_x = (world.upper_1.x - world.upper_2.x).abs();

    let viewport = box_coords.coords_viewport.clone();
    let width_upper_x = (viewport.upper_1.x - viewport.upper_2.x).abs();
    let width_upper_y = (viewport.upper_1.y - viewport.upper_2.y).abs();
    let width_lower_x = (viewport.lower_1.y - viewport.lower_2.y).abs();
    let width_lower_y = (viewport.lower_1.y - viewport.lower_2.y).abs();

    // let t = width_z > drag_threshold || width_x > drag_threshold;
    // if t {
    println!("width x: {:?}, width y: {:?}", width_upper_x, width_upper_y);
    // }
    // game_cmds.drag_select = width_z > drag_threshold || width_x > drag_threshold;
    game_cmds.drag_select = width_upper_x > drag_threshold
        || width_upper_y > drag_threshold
        || width_lower_x > drag_threshold
        || width_lower_y > drag_threshold;
}

fn handle_input(
    mut cmds: Commands,
    game_cmds: Res<GameCommands>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Left) {
        cmds.trigger(SetInitialBoxCoords);
    }

    if input.pressed(MouseButton::Left) {
        cmds.trigger(SetBoxCoordsEv);

        if game_cmds.drag_select {
            cmds.trigger(DragSelectEv);
        }
    }

    if input.just_released(MouseButton::Left) {
        cmds.trigger(ClearBoxCoordsEv);
    }
}

fn set_starting_point_drag_select_coords(
    _trigger: Trigger<SetInitialBoxCoords>,
    mut box_coords: ResMut<SelectBox>,
    mouse_coords: Res<MouseCoords>,
) {
    box_coords.coords_viewport.upper_1 = mouse_coords.viewport;
    box_coords.coords_world.upper_1 = mouse_coords.world;
}

// fn set_world_coords() {

// }

fn set_drag_select_coords(
    _trigger: Trigger<SetBoxCoordsEv>,
    mut box_coords: ResMut<SelectBox>,
    mouse_coords: Res<MouseCoords>,
    map_base_q: Query<&GlobalTransform, With<MapBase>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
) {
    let viewport = box_coords.coords_viewport.clone();
    box_coords.coords_viewport.lower_2 = mouse_coords.viewport;
    box_coords.coords_viewport.upper_2 = Vec2::new(viewport.lower_2.x, viewport.upper_1.y);
    box_coords.coords_viewport.lower_1 = Vec2::new(viewport.upper_1.x, viewport.lower_2.y);

    // WORLD
    let map_base_trans = map_base_q.single();
    let (cam, cam_trans) = cam_q.single();
    let plane_origin = map_base_trans.translation();
    let plane = InfinitePlane3d::new(map_base_trans.up());

    let Some(ray_top_end) = cam.viewport_to_world(cam_trans, box_coords.coords_viewport.upper_2)
    else {
        return;
    };

    let Some(ray_bottom_start) =
        cam.viewport_to_world(cam_trans, box_coords.coords_viewport.lower_1)
    else {
        return;
    };

    let Some(distance_bottom_start) = ray_bottom_start.intersect_plane(plane_origin, plane) else {
        return;
    };

    let Some(distance_top_end) = ray_top_end.intersect_plane(plane_origin, plane) else {
        return;
    };

    let mut top_end = ray_top_end.get_point(distance_top_end);
    let mut bottom_start = ray_bottom_start.get_point(distance_bottom_start);
    top_end.y = 0.2;
    bottom_start.y = 0.2;

    box_coords.coords_world.lower_2 = mouse_coords.world;
    box_coords.coords_world.lower_2.y = 0.2;
    box_coords.coords_world.upper_2 = top_end;
    box_coords.coords_world.lower_1 = bottom_start;
}

fn clear_drag_select_coords(
    _trigger: Trigger<ClearBoxCoordsEv>,
    mut box_coords: ResMut<SelectBox>,
) {
    box_coords.empty_coords();
}

// referenced https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn set_mouse_coords(
    mut mouse_coords: ResMut<MouseCoords>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
    map_base_q: Query<&GlobalTransform, With<MapBase>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let Some(viewport_cursor) = window_q.single().cursor_position() else {
        return;
    };
    let coords = helper(map_base_q.single(), &cam_trans, &cam, viewport_cursor);

    mouse_coords.viewport = coords.0;
    mouse_coords.world = coords.1;
}

fn helper(
    map_base_trans: &GlobalTransform,
    cam_trans: &GlobalTransform,
    cam: &Camera,
    viewport_cursor: Vec2,
) -> (Vec2, Vec3) {
    let plane_origin = map_base_trans.translation();
    let plane = InfinitePlane3d::new(map_base_trans.up());
    let ray = cam.viewport_to_world(cam_trans, viewport_cursor).unwrap();
    let distance = ray.intersect_plane(plane_origin, plane).unwrap();
    let world_cursor = ray.get_point(distance);

    return (viewport_cursor, world_cursor);
}

fn spawn_selection_box(mut cmds: Commands) {
    let gray = Color::srgba(0.68, 0.68, 0.68, 0.25);

    let selection_box = (
        NodeBundle {
            background_color: BackgroundColor(gray),
            border_color: BorderColor(DARK_GRAY.into()),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        SelectionBox,
    );

    cmds.spawn(selection_box);
}

fn draw_drag_select_box(
    mut gizmos: Gizmos,
    mut query: Query<&mut Style, With<SelectionBox>>,
    box_coords: Res<SelectBox>,
    game_cmds: Res<GameCommands>,
) {
    let mut style = query.get_single_mut().unwrap();

    if !game_cmds.drag_select {
        style.width = Val::ZERO;
        style.border = UiRect::ZERO;
        return;
    }

    let start = box_coords.coords_viewport.upper_1;
    let end = box_coords.coords_viewport.lower_2;

    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let min_y = start.y.min(end.y);
    let max_y = start.y.max(end.y);

    style.border = UiRect::all(Val::Percent(0.1));
    style.left = Val::Px(min_x);
    style.top = Val::Px(min_y);
    style.width = Val::Px(max_x - min_x);
    style.height = Val::Px(max_y - min_y);

    let color = RED;
    let world = box_coords.coords_world.clone();
    gizmos.line(world.upper_1, world.upper_2, color); // top
    gizmos.line(world.lower_1, world.lower_2, color); // bottom
    gizmos.line(world.upper_2, world.lower_2, color); // side
    gizmos.line(world.upper_1, world.lower_1, color); // side
}

pub fn handle_drag_select(
    _trigger: Trigger<DragSelectEv>,
    mut friendly_q: Query<(&Transform, &mut Selected), With<Friendly>>,
    box_coords: Res<SelectBox>,
) {
    // println!("{:?}", box_coords);

    let start = box_coords.coords_world.upper_1;
    let end = box_coords.coords_world.upper_2;

    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let min_z = start.z.min(end.z);
    let max_z = start.z.max(end.z);

    for (friendly_trans, mut selected) in friendly_q.iter_mut() {
        // check to see if units are within selection rectangle
        let unit_pos = friendly_trans.translation;
        let in_box_bounds = unit_pos.x >= min_x
            && unit_pos.x <= max_x
            && unit_pos.z >= min_z
            && unit_pos.z <= max_z;

        selected.0 = in_box_bounds;
    }
}

pub fn single_select(
    rapier_context: Res<RapierContext>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mut select_q: Query<(Entity, &mut Selected), With<Friendly>>,
    mouse_coords: Res<MouseCoords>,
    input: Res<ButtonInput<MouseButton>>,
    game_cmds: Res<GameCommands>,
) {
    if !input.just_released(MouseButton::Left) || game_cmds.drag_select {
        return;
    }

    let (cam, cam_trans) = cam_q.single();

    let Some(ray) = cam.viewport_to_world(cam_trans, mouse_coords.viewport) else {
        return;
    };

    let hit = rapier_context.cast_ray(
        ray.origin,
        ray.direction.into(),
        f32::MAX,
        true,
        QueryFilter::only_dynamic(),
    );

    if let Some((ent, _)) = hit {
        // deselect all currently selected entities
        for (selected_entity, mut selected) in select_q.iter_mut() {
            let tmp = selected_entity.index() == ent.index();
            selected.0 = tmp && !selected.0;
        }
    }
}

pub fn deselect_all(
    mut select_q: Query<&mut Selected, With<Selected>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Right) {
        for mut selected in select_q.iter_mut() {
            selected.0 = false;
        }
    }
}

fn set_selected(mut game_cmds: ResMut<GameCommands>, select_q: Query<&Selected>) {
    game_cmds.selected = false;
    for selected in select_q.iter() {
        if selected.0 {
            game_cmds.selected = true;
        }
    }
}
