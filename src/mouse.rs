use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_billboard::BillboardMeshHandle;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rts_camera::RtsCamera;

use crate::events::*;
use crate::resources::*;
use crate::utils;
use crate::*;
use crate::{components::*, CURSOR_SIZE};
use bevy_rts_pathfinding::components as pf_comps;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_select_box, spawn_cursor))
            .add_systems(PreUpdate, (set_mouse_coords, update_cursor_pos).chain())
            .add_systems(
                Update,
                (
                    update_cursor_img,
                    border_select_visibility,
                    handle_mouse_input,
                    draw_select_box,
                    set_drag_select,
                    set_selected,
                )
                    .chain(),
            )
            .observe(deselect_all)
            .observe(single_select)
            .observe(handle_drag_select)
            .observe(set_start_select_box_coords)
            .observe(set_select_box_coords)
            .observe(clear_drag_select_coords);
    }
}

fn spawn_select_box(mut cmds: Commands) {
    let select_box = (
        NodeBundle {
            background_color: BackgroundColor(COLOR_SELECT_BOX),
            border_color: BorderColor(COLOR_SELECT_BOX_BORDER.into()),
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

fn spawn_cursor(
    mut cmds: Commands,
    my_assets: Res<MyAssets>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_q.get_single_mut().unwrap();
    window.cursor.visible = false;

    let cursor = (
        ImageBundle {
            image: UiImage::new(my_assets.cursor_standard.clone()),
            style: Style {
                width: Val::Px(CURSOR_SIZE),
                height: Val::Px(CURSOR_SIZE),
                ..default()
            },
            ..default()
        },
        MyCursor::default(),
        Name::new("relocate cursor"),
    );

    cmds.spawn(cursor);
}

fn update_cursor_pos(
    mut cursor_q: Query<(&mut Style, &mut MyCursor), With<MyCursor>>,
    mouse_coords: Res<MouseCoords>,
) {
    let (mut style, cursor) = cursor_q.get_single_mut().unwrap();
    style.left = Val::Px(mouse_coords.viewport.x - cursor.size / 2.0);
    style.top = Val::Px(mouse_coords.viewport.y - cursor.size / 2.0);
}

fn handle_mouse_input(
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

        if !game_cmds.drag_select {
            cmds.trigger(SetUnitDestinationEv);
            cmds.trigger(SelectSingleUnitEv);
        }
    }

    if input.just_released(MouseButton::Right) {
        cmds.trigger(DeselectAllEv);
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
    map_base_q: Query<&GlobalTransform, With<pf_comps::MapBase>>,
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
    select_box.world.start_1 = utils::get_world_coords(&map_base, cam_trans, cam, viewport.start_1);
    select_box.world.start_2 = utils::get_world_coords(&map_base, cam_trans, cam, viewport.start_2);
    select_box.world.end_1 = utils::get_world_coords(&map_base, cam_trans, cam, viewport.end_1);
    select_box.world.end_2 = utils::get_world_coords(&map_base, cam_trans, cam, viewport.end_2);
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
    map_base_q: Query<&GlobalTransform, With<pf_comps::MapBase>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let Some(viewport_cursor) = window_q.single().cursor_position() else {
        return;
    };
    let coords = utils::get_world_coords(map_base_q.single(), &cam_trans, &cam, viewport_cursor);

    mouse_coords.viewport = viewport_cursor;
    mouse_coords.world = coords;
}

fn draw_select_box(
    mut _gizmos: Gizmos,
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
    // _gizmos.line(world.start_1, world.start_2, color); // top
    // _gizmos.line(world.end_1, world.end_2, color); // bottom
    // _gizmos.line(world.start_2, world.end_2, color); // side
    // _gizmos.line(world.start_1, world.end_1, color); // side
}

pub fn handle_drag_select(
    _trigger: Trigger<HandleDragSelectEv>,
    mut cmds: Commands,
    mut unit_q: Query<(Entity, &Transform), With<Unit>>,
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
    for (friendly_ent, friendly_trans) in unit_q.iter_mut() {
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
        if in_box_bounds {
            cmds.entity(friendly_ent).insert(pf_comps::Selected);
        } else {
            cmds.entity(friendly_ent).remove::<pf_comps::Selected>();
        }
    }
}

pub fn update_cursor_img(
    game_cmds: Res<GameCommands>,
    mut cursor_state: ResMut<CursorState>,
    rapier_context: Res<RapierContext>,
    my_assets: Res<MyAssets>,
    mouse_coords: Res<MouseCoords>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mut cursor_q: Query<(&mut UiImage, &mut MyCursor)>,
    mut select_q: Query<Entity, With<Unit>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let hit = utils::cast_ray(rapier_context, &cam, &cam_trans, mouse_coords.viewport);

    if let Some((ent, _)) = hit {
        for selected_entity in select_q.iter_mut() {
            let tmp = selected_entity.index() == ent.index();

            if tmp && !game_cmds.drag_select {
                *cursor_state = CursorState::Select;
            }
        }
    } else if game_cmds.selected && !game_cmds.drag_select {
        *cursor_state = CursorState::Relocate;
    } else {
        *cursor_state = CursorState::Standard;
    }

    let (mut img, mut cursor) = cursor_q.get_single_mut().unwrap();
    // println!("Change Cursor: {:?}", game_cmds.cursor_state);

    match *cursor_state {
        CursorState::Relocate => cursor.img = my_assets.cursor_relocate.clone(),
        CursorState::Standard => cursor.img = my_assets.cursor_standard.clone(),
        CursorState::Select => cursor.img = my_assets.cursor_select.clone(),
    }

    img.texture = cursor.img.clone();
}

pub fn single_select(
    _trigger: Trigger<SelectSingleUnitEv>,
    rapier_context: Res<RapierContext>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mut unit_q: Query<Entity, With<Unit>>,
    mut cmds: Commands,
    mouse_coords: Res<MouseCoords>,
) {
    let (cam, cam_trans) = cam_q.single();
    let hit = utils::cast_ray(rapier_context, &cam, &cam_trans, mouse_coords.viewport);

    // deselect all currently pf_comps::selected entities
    if let Some((ent, _)) = hit {
        for selected_entity in unit_q.iter_mut() {
            let tmp = selected_entity.index() == ent.index();

            if !tmp {
                cmds.entity(selected_entity).remove::<pf_comps::Selected>();
            } else {
                cmds.entity(selected_entity).insert(pf_comps::Selected);
            }
        }
    }
}

pub fn deselect_all(
    _trigger: Trigger<DeselectAllEv>,
    mut cmds: Commands,
    mut select_q: Query<Entity, With<pf_comps::Selected>>,
) {
    for entity in select_q.iter_mut() {
        cmds.entity(entity).remove::<pf_comps::Selected>();
    }
}

fn set_selected(mut game_cmds: ResMut<GameCommands>, select_q: Query<&pf_comps::Selected>) {
    game_cmds.selected = false;
    if !select_q.is_empty() {
        game_cmds.selected = true;
    }
}

fn border_select_visibility(
    selected_units_q: Query<Entity, With<pf_comps::Selected>>,
    non_selected_units_q: Query<Entity, Without<pf_comps::Selected>>,
    mut border_select_q: Query<
        (&mut BillboardMeshHandle, &UnitBorderBoxImg),
        With<UnitBorderBoxImg>,
    >,
    children_q: Query<&Children>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // make border visible for selected entities
    for entity in selected_units_q.iter() {
        for child in children_q.iter_descendants(entity) {
            if let Ok((mut billboard_mesh, border)) = border_select_q.get_mut(child) {
                let border_xy = Vec2::new(border.width, border.height);
                *billboard_mesh = BillboardMeshHandle(meshes.add(Rectangle::from_size(border_xy)));
            }
        }
    }

    // make border invisible for unselected entities
    for entity in non_selected_units_q.iter() {
        for child in children_q.iter_descendants(entity) {
            if let Ok((mut billboard_mesh, _border)) = border_select_q.get_mut(child) {
                let border_xy = Vec2::new(0.0, 0.0);
                *billboard_mesh = BillboardMeshHandle(meshes.add(Rectangle::from_size(border_xy)));
            }
        }
    }
}
