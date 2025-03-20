use bevy::winit::cursor::{CursorIcon, CustomCursor};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::plugin::RapierContext;
use bevy_rts_camera::RtsCamera;
use core::f32;
use std::f32::consts::FRAC_PI_2;

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::utils;
use crate::*;
use bevy_rts_pathfinding::components::{self as pf_comps};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_drag_select_box)
            .add_systems(PreUpdate, set_mouse_coords)
            .add_systems(
                Update,
                (
                    set_is_any_selected,
                    mouse_input,
                    draw_drag_select_box,
                    set_drag_select,
                    sync_select_border_with_unit,
                    update_cursor_img,
                )
                    .chain(),
            )
            .add_observer(deselect_all)
            .add_observer(single_select)
            .add_observer(handle_drag_select)
            .add_observer(set_start_drag_select_box_coords)
            .add_observer(set_drag_select_box_coords)
            .add_observer(clear_drag_select_coords);
    }
}

fn sync_select_border_with_unit(
    mut q_border: Query<(&mut Node, &UnitSelectBorder)>,
    q_unit_transform: Query<(&Transform, &BorderSize), With<Unit>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let window = window_q.single();

    // For this example we assume a perspective camera with a 90° vertical FOV.
    // In a real app, you’d query for your camera's actual fov.
    let fov_y = FRAC_PI_2;

    for (mut style, border) in q_border.iter_mut() {
        let Ok((unit_transform, border_size)) = q_unit_transform.get(border.0) else {
            continue;
        };

        // Get the unit's center in screen space.
        let center_screen = match cam.world_to_viewport(cam_trans, unit_transform.translation) {
            Ok(pos) => pos,
            Err(_) => continue,
        };

        // Compute the distance from the camera to the unit.
        let distance = cam_trans.translation().distance(unit_transform.translation);

        // Use the formula:
        // scale = (window_height/2) / (distance * tan(fov_y/2))
        let window_height = window.physical_height() as f32;
        let scale = (window_height / 2.0) / (distance * (fov_y / 2.0).tan());

        // Compute the screen-space width and height of the unit.
        let screen_width = border_size.0.x * scale;
        let screen_height = border_size.0.y * scale;

        // Position the border so that its center aligns with the unit's screen center.
        style.left = Val::Px(center_screen.x - screen_width / 2.0);
        style.top = Val::Px(center_screen.y - screen_height / 2.0);
        style.width = Val::Px(screen_width);
        style.height = Val::Px(screen_height);
    }
}

fn spawn_drag_select_box(mut cmds: Commands) {
    let select_box = (
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(COLOR_SELECT_BOX),
        BorderColor(COLOR_SELECT_BOX_BORDER.into()),
        SelectionBox,
    );

    cmds.spawn(select_box);
}

fn mouse_input(
    mut cmds: Commands,
    game_cmds: Res<GameCommands>,
    input: Res<ButtonInput<MouseButton>>,
    q_rapier: Query<&RapierContext, With<DefaultRapierContext>>,
    q_cam: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mouse_coords: Res<MouseCoords>,
    q_unit: Query<Entity, With<Unit>>,
) {
    if game_cmds.hvr_cmd_interface {
        return;
    }

    cmds.trigger(SetDragSelectEv);

    if input.just_pressed(MouseButton::Left) {
        cmds.trigger(SetStartBoxCoordsEv);
        return;
    }

    if input.pressed(MouseButton::Left) {
        cmds.trigger(SetBoxCoordsEv);

        if game_cmds.drag_select {
            cmds.trigger(HandleDragSelectEv);
        }

        return;
    }

    if input.just_released(MouseButton::Left) {
        cmds.trigger(ClearBoxCoordsEv);

        if !game_cmds.drag_select {
            let Ok(rapier_ctx) = q_rapier.get_single() else {
                return;
            };

            let (cam, cam_trans) = q_cam.single();
            let hit = utils::cast_ray(rapier_ctx, &cam, &cam_trans, mouse_coords.viewport);

            let mut hit_ent_option = None;
            if let Some((hit_ent, _)) = hit {
                if let Ok(_) = q_unit.get(hit_ent) {
                    hit_ent_option = Some(hit_ent);
                }
            }

            if !game_cmds.is_any_selected || hit_ent_option.is_some() {
                cmds.trigger(DeselectAllEv);

                if let Some(hit_ent) = hit_ent_option {
                    cmds.trigger(SelectSingleUnitEv(hit_ent));
                }
            } else if hit_ent_option.is_none() {
                cmds.trigger(SetUnitDestinationEv);
            }
        }

        return;
    }

    if input.just_released(MouseButton::Right) {
        cmds.trigger(DeselectAllEv);
    }
}

fn set_drag_select(box_coords: Res<SelectBox>, mut game_cmds: ResMut<GameCommands>) {
    if game_cmds.hvr_cmd_interface {
        return;
    }

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

fn set_start_drag_select_box_coords(
    _trigger: Trigger<SetStartBoxCoordsEv>,
    mut box_coords: ResMut<SelectBox>,
    mouse_coords: Res<MouseCoords>,
) {
    box_coords.viewport.initialize_coords(mouse_coords.viewport);
    box_coords.world.initialize_coords(mouse_coords.world);
}

fn set_drag_select_box_coords(
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
    if let Some(coords) = utils::get_world_coords(&map_base, cam_trans, cam, viewport.start_1) {
        select_box.world.start_1 = coords;
    }

    if let Some(coords) = utils::get_world_coords(&map_base, cam_trans, cam, viewport.start_2) {
        select_box.world.start_2 = coords;
    }

    if let Some(coords) = utils::get_world_coords(&map_base, cam_trans, cam, viewport.end_1) {
        select_box.world.end_1 = coords;
    }

    if let Some(coords) = utils::get_world_coords(&map_base, cam_trans, cam, viewport.end_2) {
        select_box.world.end_2 = coords;
    }
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
    cam_q: Query<(&Camera, &GlobalTransform), With<pf_comps::GameCamera>>,
    map_base_q: Query<&GlobalTransform, With<pf_comps::MapBase>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let Some(viewport_cursor) = window_q.single().cursor_position() else {
        return;
    };
    let coords = utils::get_world_coords(map_base_q.single(), &cam_trans, &cam, viewport_cursor);

    mouse_coords.viewport = viewport_cursor;

    if let Some(coords) = coords {
        mouse_coords.world = coords;
    }
}

fn draw_drag_select_box(
    mut _gizmos: Gizmos,
    mut q_select_box: Query<&mut Node, With<SelectionBox>>,
    box_coords: Res<SelectBox>,
    game_cmds: Res<GameCommands>,
) {
    let Ok(mut style) = q_select_box.get_single_mut() else {
        return;
    };

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
    q_selected: Query<&Selected>,
    my_assets: Res<MyAssets>,
    q_border: Query<(Entity, &UnitSelectBorder)>,
) {
    fn cross_product(v1: Vec3, v2: Vec3) -> f32 {
        v1.x * v2.z - v1.z * v2.x
    }

    // 4 corners of select box
    let a = box_coords.world.start_1;
    let b = box_coords.world.start_2;
    let c = box_coords.world.end_2;
    let d = box_coords.world.end_1;

    let border = |ent: Entity| -> (UnitSelectBorder, ImageNode, Name) {
        (
            UnitSelectBorder(ent),
            ImageNode::new(my_assets.images.select_border.clone()),
            Name::new("Unit Select Border"),
        )
    };

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
            if q_selected.get(friendly_ent).is_err() {
                cmds.entity(friendly_ent).insert(Selected);
                cmds.spawn(border(friendly_ent));
            }
        } else {
            // Collect all border entities for this selected unit.
            let borders_to_despawn: Vec<Entity> = q_border
                .iter()
                .filter_map(|(border_entity, border_comp)| {
                    if border_comp.0 == friendly_ent {
                        Some(border_entity)
                    } else {
                        None
                    }
                })
                .collect();

            // Despawn the collected border entities.
            for border_entity in borders_to_despawn {
                cmds.entity(border_entity).despawn_recursive();
            }

            cmds.entity(friendly_ent).remove::<Selected>();
        }
    }
}

pub fn update_cursor_img(
    game_cmds: Res<GameCommands>,
    mut cursor_state: ResMut<CursorState>,
    my_assets: Res<MyAssets>,
    mouse_coords: Res<MouseCoords>,
    q_rapier: Query<&RapierContext, With<DefaultRapierContext>>,
    q_cam: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    q_unit: Query<&Unit>,
    mut q_cursor: Query<&mut CursorIcon>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(rapier_ctx) = q_rapier.get_single() else {
        return;
    };

    let Ok(mut cursor) = q_cursor.get_single_mut() else {
        return;
    };

    let Ok(mut window) = q_window.get_single_mut() else {
        return;
    };

    let (cam, cam_trans) = q_cam.single();

    let hit = utils::cast_ray(rapier_ctx, &cam, &cam_trans, mouse_coords.viewport);

    if hit.is_some()
        // && !game_cmds.is_any_selected
        && !game_cmds.drag_select
        && *cursor_state != CursorState::Build
        && !game_cmds.hvr_cmd_interface
    {
        if let Some((hit_ent, _)) = hit {
            if let Ok(_) = q_unit.get(hit_ent) {
                *cursor_state = CursorState::Select;
            }
        }
    } else if game_cmds.is_any_selected && !game_cmds.drag_select {
        *cursor_state = CursorState::Relocate;
    } else if !game_cmds.drag_select && *cursor_state != CursorState::Build {
        *cursor_state = CursorState::Standard;
    }

    let img: Handle<Image>;
    let hotspot: (u16, u16);
    match *cursor_state {
        CursorState::Relocate => {
            window.cursor_options.visible = true;
            img = my_assets.images.cursor_relocate.clone();
            hotspot = (2, 2)
        }
        CursorState::Standard => {
            window.cursor_options.visible = true;
            img = my_assets.images.cursor_standard.clone();
            hotspot = (0, 0)
        }
        CursorState::Select => {
            window.cursor_options.visible = true;
            img = my_assets.images.cursor_select.clone();
            hotspot = (25, 25)
        }
        CursorState::Build => {
            window.cursor_options.visible = false;
            img = my_assets.images.cursor_relocate.clone();
            hotspot = (0, 0)
        }
    }

    *cursor = CursorIcon::Custom(CustomCursor::Image {
        handle: img,
        hotspot,
    });
}

pub fn single_select(
    trigger: Trigger<SelectSingleUnitEv>,
    mut cmds: Commands,
    game_cmds: Res<GameCommands>,
    my_assets: Res<MyAssets>,
) {
    if game_cmds.hvr_cmd_interface {
        return;
    }

    let ent = trigger.0;

    // Closure that creates a new border for a given unit.
    let border = |ent: Entity| -> (UnitSelectBorder, ImageNode) {
        (
            UnitSelectBorder(ent),
            ImageNode {
                image: my_assets.images.select_border.clone(),
                ..default()
            },
        )
    };

    cmds.entity(ent).insert(Selected);
    cmds.spawn(border(ent));
}

pub fn deselect_all(
    _trigger: Trigger<DeselectAllEv>,
    mut cmds: Commands,
    mut select_q: Query<Entity, With<Selected>>,
    mut q_border: Query<Entity, With<UnitSelectBorder>>,
) {
    for entity in select_q.iter_mut() {
        cmds.entity(entity).remove::<Selected>();
    }

    for border_ent in q_border.iter_mut() {
        cmds.entity(border_ent).despawn_recursive();
    }
}

fn set_is_any_selected(q_selected: Query<&Selected>, mut game_cmds: ResMut<GameCommands>) {
    game_cmds.is_any_selected = q_selected.iter().next().is_some();
}
