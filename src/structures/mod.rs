use std::f32::consts::FRAC_PI_2;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_mod_outline::AsyncSceneInheritOutline;
use bevy_mod_outline::OutlineMode;
use bevy_mod_outline::OutlineVolume;
use bevy_rapier3d::prelude::*;
use bevy_rts_camera::RtsCamera;
use bevy_rts_pathfinding::components::RtsObjSize;
use bevy_rts_pathfinding::components::{self as pf_comps};
use vehicle_depot::VehicleDepotPlugin;

use crate::asset_manager::audio::*;
use crate::bank::*;
use crate::components::structures::*;
use crate::events::*;
use crate::resources::structures::StructuresBuilt;
use crate::resources::*;
use crate::utils;

mod vehicle_depot;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VehicleDepotPlugin)
            .add_systems(
                Update,
                (
                    primary_structure_txt,
                    mark_structure_built,
                    sync_placeholder,
                    deselect_if_any_unit_is_selected,
                    validate_structure_placement,
                    place_structure.after(validate_structure_placement),
                ),
            )
            .add_observer(select_structure)
            .add_observer(deselect);
    }
}

// modifies the 'StructuresBuilt' resource, whenever a structure is placed or removed (destroyed)
pub fn mark_structure_built(
    mut structures_built: ResMut<StructuresBuilt>,
    q_structure_added: Query<&StructureType, Added<Structure>>,
) {
    for structure in q_structure_added.iter() {
        match structure {
            StructureType::Cannon => structures_built.cannon += 1,
            StructureType::Barracks => structures_built.barracks += 1,
            StructureType::VehicleDepot => structures_built.vehicle_depot += 1,
            StructureType::ResearchCenter => structures_built.research_center += 1,
            StructureType::SatelliteDish => structures_built.satellite_dish += 1,
        }
    }
}

fn select_structure(
    trigger: Trigger<SelectStructureEv>,
    dbg: Res<DbgOptions>,
    mut cmds: Commands,
    game_cmds: Res<GameCommands>,
    mut q: Query<Entity, With<NewlyPlacedStructure>>,
) {
    // Hack. This is used to prevent a newly placed structure from automatically being selected
    if let Ok(ent) = q.get_single_mut() {
        cmds.entity(ent).remove::<NewlyPlacedStructure>();
        return;
    }

    dbg.print("Structure selected");

    if game_cmds.hvr_cmd_interface {
        return;
    }

    let structure_ent = trigger.0;

    let outline = (
        OutlineVolume {
            visible: true,
            colour: Color::WHITE,
            width: 3.0,
        },
        OutlineMode::FloodFlat,
        AsyncSceneInheritOutline::default(),
    );

    cmds.entity(structure_ent)
        .insert((SelectedStructure, outline));
}

fn place_structure(
    mut cmds: Commands,
    mut q_placeholder: Query<
        (
            Entity,
            &StructurePlaceholder,
            &StructureType,
            &mut RigidBody,
            &mut SceneRoot,
            &pf_comps::RtsObjSize,
        ),
        With<StructurePlaceholder>,
    >,
    dbg: Res<DbgOptions>,
    input: Res<ButtonInput<MouseButton>>,
    mut cursor_state: ResMut<CursorState>,
    my_assets: Res<MyAssets>,
    my_audio: Res<MyAudio>,
    game_cmds: Res<GameCommands>,
) {
    if *cursor_state != CursorState::Build || game_cmds.hvr_cmd_interface {
        return;
    }

    let Ok((placeholder_ent, placeholder, structure, mut rb, mut scene, _size)) =
        q_placeholder.get_single_mut()
    else {
        return;
    };

    if input.just_pressed(MouseButton::Left) && placeholder.is_valid {
        *cursor_state = CursorState::Standard;
        structure.place(placeholder_ent, &my_assets, &mut scene, &mut rb, &mut cmds);

        // Adjust bank
        cmds.trigger(AdjustFundsEv(-structure.cost()));

        // place structure audio
        let audio = AudioPlayer::new(my_audio.place_structure.clone());
        cmds.spawn(audio);

        dbg.print("Build Structure");
    }
}

fn validate_structure_placement(
    q_rapier: Query<&RapierContext, With<DefaultRapierContext>>,
    mut q_placeholder: Query<(Entity, &mut StructurePlaceholder, &mut SceneRoot)>,
    q_collider: Query<&Collider, With<pf_comps::MapBase>>,
    my_assets: Res<MyAssets>,
) {
    let Ok((placeholder_ent, mut placeholder, mut scene)) = q_placeholder.get_single_mut() else {
        return;
    };

    let Ok(rapier_ctx) = q_rapier.get_single() else {
        return;
    };

    let mut is_colliding = false;
    for (ent_1, ent_2, intersect) in rapier_ctx.intersection_pairs_with(placeholder_ent) {
        // exclude any collisions with the map base
        if q_collider.get(ent_1).is_ok() || q_collider.get(ent_2).is_ok() {
            continue;
        }

        is_colliding = intersect;
    }

    if is_colliding {
        placeholder.is_valid = false;
        placeholder
            .structure
            .invalid_placement(&my_assets, &mut scene);
    } else {
        placeholder.is_valid = true;
        placeholder
            .structure
            .valid_placement(&my_assets, &mut scene);
    }
}

fn sync_placeholder(
    mut q_placeholder: Query<(&mut Transform, &pf_comps::RtsObjSize), With<StructurePlaceholder>>,
    mut cam_q: Query<(&Camera, &GlobalTransform), With<pf_comps::GameCamera>>,
    map_base_q: Query<&GlobalTransform, With<pf_comps::MapBase>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let Ok((mut transform, size)) = q_placeholder.get_single_mut() else {
        return;
    };

    const ROTATION_SPEED: f32 = 0.3;
    for event in mouse_wheel_events.read() {
        transform.rotate_y(event.y * ROTATION_SPEED);
    }

    let (cam, cam_trans) = cam_q.single_mut();

    let Some(viewport_cursor) = q_window.single().cursor_position() else {
        return;
    };

    let coords = utils::get_world_coords(map_base_q.single(), &cam_trans, &cam, viewport_cursor);
    if let Some(coords) = coords {
        transform.translation = coords;
        transform.translation.y = size.0.y / 2.0;
    }
}

fn deselect_if_any_unit_is_selected(
    mut cmds: Commands,
    game_cmds: Res<GameCommands>,
    q_selected_structure: Query<Entity, With<SelectedStructure>>,
) {
    if !game_cmds.is_any_unit_selected {
        return;
    }

    for structure_ent in q_selected_structure.iter() {
        cmds.entity(structure_ent).remove::<(
            SelectedStructure,
            OutlineVolume,
            OutlineMode,
            AsyncSceneInheritOutline,
        )>();
    }
}

fn deselect(
    _trigger: Trigger<DeselectAllEv>,
    mut cmds: Commands,
    mut q_selected_structure: Query<Entity, With<SelectedStructure>>,
) {
    for structure_ent in q_selected_structure.iter_mut() {
        cmds.entity(structure_ent).remove::<(
            SelectedStructure,
            OutlineVolume,
            OutlineMode,
            AsyncSceneInheritOutline,
        )>();
    }
}

#[derive(Component)]
pub struct PrimaryStructureTxt;

fn primary_structure_txt(
    mut cmds: Commands,
    q_selected_structure: Query<
        (&Transform, &RtsObjSize, &PrimaryStructure),
        With<SelectedStructure>,
    >,
    mut q_primary_structure_txt: Query<&mut Node, With<PrimaryStructureTxt>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut style) = q_primary_structure_txt.get_single_mut() {
        let (cam, cam_trans) = cam_q.single();
        let window = window_q.single();

        let Ok((transform, obj_size, _primary_structure)) = q_selected_structure.get_single()
        else {
            return;
        };

        // Get the unit's center in screen space.
        let center_screen = match cam.world_to_viewport(cam_trans, transform.translation) {
            Ok(pos) => pos,
            Err(_) => return,
        };

        // Compute the distance from the camera to the unit.
        let distance = cam_trans.translation().distance(transform.translation);

        // Use the formula:
        // scale = (window_height/2) / (distance * tan(fov_y/2))
        let window_height = window.physical_height() as f32;
        let scale = (window_height / 2.0) / (distance * (FRAC_PI_2 / 2.0).tan());

        // Compute the screen-space width and height of the unit.
        let screen_width = obj_size.0.x * scale;
        let screen_height = obj_size.0.y * scale;

        // Position the border so that its center aligns with the unit's screen center.
        style.left = Val::Px(center_screen.x - screen_width / 2.0);
        style.top = Val::Px(center_screen.y - screen_height / 2.0);
        style.width = Val::Px(screen_width);
        style.height = Val::Px(screen_height);
    } else {
        println!("spawnings txt container");
        let txt_ctr = (
            PrimaryStructureTxt,
            Text::new("Active"),
            Node::default(),
            Name::new("Primary Structure Container"),
        );

        cmds.spawn(txt_ctr);
    }
}
