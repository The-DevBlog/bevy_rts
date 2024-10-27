use bevy::color::palettes::css::DARK_GRAY;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext};
use bevy_rts_camera::RtsCamera;

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::tank::set_unit_destination;

const SELECT_BOX_COLOR: Color = Color::srgba(0.68, 0.68, 0.68, 0.25);
const SELECT_BOX_BORDER_COLOR: Srgba = DARK_GRAY;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_select_box)
            .add_systems(
                Update,
                (
                    set_mouse_coords,
                    handle_input,
                    draw_select_box,
                    single_select,
                    set_drag_select,
                    set_selected,
                    deselect_all,
                )
                    .chain()
                    .after(set_unit_destination),
            )
            .observe(handle_drag_select)
            .observe(set_start_select_box_coords)
            .observe(set_select_box_coords)
            .observe(clear_drag_select_coords);
    }
}

fn set_drag_select(box_coords: Res<SelectBox>, mut game_cmds: ResMut<GameCommands>) {
    let drag_threshold = 2.5;
    let viewport = box_coords.viewport.clone();

    let widths = [
        (viewport.start_1.x - viewport.start_2.x).abs(),
        (viewport.start_1.y - viewport.start_2.y).abs(),
        (viewport.end_1.y - viewport.end_2.y).abs(),
        (viewport.end_1.y - viewport.end_2.y).abs(),
    ];

    game_cmds.drag_select = widths.iter().any(|&width| width > drag_threshold);
}

fn handle_input(
    mut cmds: Commands,
    game_cmds: Res<GameCommands>,
    input: Res<ButtonInput<MouseButton>>,
) {
    cmds.trigger(SetDragSelectEv);

    if input.just_pressed(MouseButton::Left) {
        cmds.trigger(SetStartBoxCoordsEv);
    }

    if input.pressed(MouseButton::Left) {
        cmds.trigger(SetBoxCoordsEv);

        if game_cmds.drag_select {
            cmds.trigger(HandleDragSelectEv);
        }
    }

    if input.just_released(MouseButton::Left) {
        cmds.trigger(ClearBoxCoordsEv);
    }
}

fn set_start_select_box_coords(
    _trigger: Trigger<SetStartBoxCoordsEv>,
    mut box_coords: ResMut<SelectBox>,
    mouse_coords: Res<MouseCoords>,
) {
    box_coords.viewport.initialize_coords(mouse_coords.viewport);
    box_coords.world.initialize_coords(mouse_coords.world);
}

fn set_select_box_coords(
    _trigger: Trigger<SetBoxCoordsEv>,
    mut select_box: ResMut<SelectBox>,
    mouse_coords: Res<MouseCoords>,
    map_base_q: Query<&GlobalTransform, With<MapBase>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
) {
    let viewport = select_box.viewport.clone();
    select_box.viewport.end_2 = mouse_coords.viewport;
    select_box.viewport.start_2 = Vec2::new(viewport.end_2.x, viewport.start_1.y);
    select_box.viewport.end_1 = Vec2::new(viewport.start_1.x, viewport.end_2.y);

    let map_base = map_base_q.single();
    let (cam, cam_trans) = cam_q.single();

    // Convert each viewport corner to world coordinates based on the current camera view
    let viewport = select_box.viewport.clone();
    select_box.world.start_1 = get_world_coords(&map_base, cam_trans, cam, viewport.start_1);
    select_box.world.start_2 = get_world_coords(&map_base, cam_trans, cam, viewport.start_2);
    select_box.world.end_1 = get_world_coords(&map_base, cam_trans, cam, viewport.end_1);
    select_box.world.end_2 = get_world_coords(&map_base, cam_trans, cam, viewport.end_2);
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
    let coords = get_world_coords(map_base_q.single(), &cam_trans, &cam, viewport_cursor);

    mouse_coords.viewport = viewport_cursor;
    mouse_coords.world = coords;
}

fn spawn_select_box(mut cmds: Commands) {
    let select_box = (
        NodeBundle {
            background_color: BackgroundColor(SELECT_BOX_COLOR),
            border_color: BorderColor(SELECT_BOX_BORDER_COLOR.into()),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        SelectionBox,
    );

    cmds.spawn(select_box);
}

fn draw_select_box(
    // mut gizmos: Gizmos,
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

    let start = box_coords.viewport.start_1;
    let end = box_coords.viewport.end_2;

    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let min_y = start.y.min(end.y);
    let max_y = start.y.max(end.y);

    style.border = UiRect::all(Val::Percent(0.1));
    style.left = Val::Px(min_x);
    style.top = Val::Px(min_y);
    style.width = Val::Px(max_x - min_x);
    style.height = Val::Px(max_y - min_y);

    // debug purposes only. This will draw the select box on the 3d world
    // let color = RED;
    // let world = box_coords.world.clone();
    // gizmos.line(world.start_1, world.start_2, color); // top
    // gizmos.line(world.end_1, world.end_2, color); // bottom
    // gizmos.line(world.start_2, world.end_2, color); // side
    // gizmos.line(world.start_1, world.end_1, color); // side
}

pub fn handle_drag_select(
    _trigger: Trigger<HandleDragSelectEv>,
    mut friendly_q: Query<(&Transform, &mut Selected), With<Friendly>>,
    box_coords: Res<SelectBox>,
) {
    fn cross_product(v1: Vec3, v2: Vec3) -> f32 {
        v1.x * v2.z - v1.z * v2.x
    }

    // 4 corners of select box
    let a = box_coords.world.start_1;
    let b = box_coords.world.start_2;
    let c = box_coords.world.end_2;
    let d = box_coords.world.end_1;

    // check to see if units are within selection rectangle
    for (friendly_trans, mut selected) in friendly_q.iter_mut() {
        let unit_pos = friendly_trans.translation;

        // Calculate cross products for each edge
        let ab = b - a;
        let ap = unit_pos - a;
        let bc = c - b;
        let bp = unit_pos - b;
        let cd = d - c;
        let cp = unit_pos - c;
        let da = a - d;
        let dp = unit_pos - d;

        let cross_ab_ap = cross_product(ab, ap);
        let cross_bc_bp = cross_product(bc, bp);
        let cross_cd_cp = cross_product(cd, cp);
        let cross_da_dp = cross_product(da, dp);

        // Check if all cross products have the same sign
        let in_box_bounds = (cross_ab_ap > 0.0
            && cross_bc_bp > 0.0
            && cross_cd_cp > 0.0
            && cross_da_dp > 0.0)
            || (cross_ab_ap < 0.0 && cross_bc_bp < 0.0 && cross_cd_cp < 0.0 && cross_da_dp < 0.0);

        // Set the selection status
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

// helper function
fn get_world_coords(
    map_base_trans: &GlobalTransform,
    cam_trans: &GlobalTransform,
    cam: &Camera,
    viewport_pos: Vec2,
) -> Vec3 {
    let plane_origin = map_base_trans.translation();
    let plane = InfinitePlane3d::new(map_base_trans.up());
    let ray = cam.viewport_to_world(cam_trans, viewport_pos).unwrap();
    let distance = ray.intersect_plane(plane_origin, plane).unwrap();
    return ray.get_point(distance);
}
