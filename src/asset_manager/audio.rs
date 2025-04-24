use bevy::prelude::*;
// use bevy_kira_audio::*;
use rand::seq::IndexedRandom;
use std::{collections::HashMap, path::PathBuf};
use strum_macros::EnumString;
use walkdir::WalkDir;

use crate::events::*;
use crate::resources::DbgOptions;
use crate::units::components::*;
// use bevy_rts_pathfinding::components as pf_comps;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyAudio>()
            .add_systems(Startup, load_assets)
            // .add_systems(Update, unit_move_audio)
            .add_observer(unit_audio)
            .add_observer(multiple_select)
            .add_observer(single_select)
            .add_observer(relocate_cmd);
    }
}

#[derive(Event)]
pub struct UnitAudioEv {
    pub cmd: AudioCmd,
    pub unit: UnitType,
}

impl UnitAudioEv {
    pub fn new(cmd: AudioCmd, unit: UnitType) -> Self {
        Self { cmd, unit }
    }
}

#[derive(Debug, EnumString, Hash, PartialEq, Eq)]
pub enum AudioCmd {
    #[strum(serialize = "move")]
    Relocate,
    #[strum(serialize = "select")]
    Select,
    #[strum(serialize = "ready")]
    Ready,
}

#[derive(Resource, Default)]
pub struct MyAudio {
    pub place_structure: Handle<bevy::prelude::AudioSource>,
    pub unit_cmds: HashMap<UnitType, HashMap<AudioCmd, Vec<Handle<bevy::prelude::AudioSource>>>>,
}

// #[derive(Default)]
// pub struct Sfx {
//     pub moving_rifleman: SfxOptions,
//     pub moving_tank_gen_1: SfxOptions,
//     pub moving_tank_gen_2: SfxOptions,
//     pub moving_artillery: SfxOptions,
// }

// impl Sfx {
//     pub fn get_moving_handle(&self, unit_type: &UnitType) -> Handle<bevy_kira_audio::AudioSource> {
//         match unit_type {
//             UnitType::TankGen1 => self.moving_tank_gen_1.source.clone(),
//             UnitType::TankGen2 => self.moving_tank_gen_2.source.clone(),
//             UnitType::Artillery => self.moving_artillery.source.clone(),
//             UnitType::Rifleman => self.moving_rifleman.source.clone(),
//         }
//     }
// }

// #[derive(Default)]
// pub struct SfxOptions {
//     pub source: Handle<bevy_kira_audio::AudioSource>,
//     // pub instance: Handle<bevy_kira_audio::AudioInstance>,
// }

fn load_assets(mut my_audio: ResMut<MyAudio>, assets: Res<AssetServer>, dbg: Res<DbgOptions>) {
    // 1) one-off sound
    my_audio.place_structure = assets.load("audio/place_structure.ogg");

    // 2) recursively load everything under `audio/unit_cmds`
    let entries = load_folder_recursive("audio/unit_cmds", &assets, &dbg);

    for (asset_path, handle) in entries {
        let parts: Vec<_> = asset_path.split('/').collect();
        if parts.len() < 5 {
            dbg.print(&format!("unexpected asset path: {}", asset_path));
            continue;
        }

        let unit_str = parts[2]; // e.g. "tank_gen_1"
        let cmd_str = parts[3]; // e.g. "select"

        match (unit_str.parse::<UnitType>(), cmd_str.parse::<AudioCmd>()) {
            (Ok(unit), Ok(cmd)) => {
                my_audio
                    .unit_cmds
                    .entry(unit)
                    .or_default()
                    .entry(cmd)
                    .or_default()
                    .push(handle.clone());
            }
            _ => {
                dbg.print(&format!(
                    "skipping unknown unit/cmd: {}/{}",
                    unit_str, cmd_str
                ));
            }
        }
    }
}

/// Walk `assets/<folder>` and return (relative asset path, handle) tuples for every .ogg
fn load_folder_recursive(
    folder: &str,
    assets: &AssetServer,
    dbg: &DbgOptions,
) -> Vec<(String, Handle<bevy::prelude::AudioSource>)> {
    // where Cargo.toml lives
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let assets_root = PathBuf::from(manifest_dir).join("assets");
    let search_dir = assets_root.join(folder);

    let mut results = Vec::new();
    for entry in WalkDir::new(&search_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ogg") {
            // strip off "…/assets/" so we get e.g. "audio/unit_cmds/…"
            if let Ok(rel) = path.strip_prefix(&assets_root) {
                let asset_path = rel.to_string_lossy().replace("\\", "/");
                dbg.print(&format!("Loading asset: {}", asset_path));
                let handle = assets.load(&asset_path);
                results.push((asset_path, handle));
            }
        }
    }

    results
}

fn unit_audio(trigger: Trigger<UnitAudioEv>, mut cmds: Commands, my_audio: Res<MyAudio>) {
    let UnitAudioEv { unit, cmd } = trigger.event();

    // 1) Get the map of commands for this unit
    if let Some(cmd_map) = my_audio.unit_cmds.get(unit) {
        // 2) Get the Vec<Handle<…>> for this AudioCmd
        if let Some(handles) = cmd_map.get(cmd) {
            // 3) Pick one at random
            if let Some(handle) = handles.choose(&mut rand::rng()) {
                cmds.spawn(AudioPlayer::new(handle.clone()));
            } else {
                warn!("No audio handles for {:?} {:?}", unit.name(), cmd);
            }
        } else {
            warn!("No audio command {:?} for unit {:?}", cmd, unit.name());
        }
    } else {
        warn!("No audio map for unit {:?}", unit.name());
    }
}

fn multiple_select(
    _trigger: Trigger<SelectMultipleUnitEv>,
    mut cmds: Commands,
    q_units: Query<(Entity, &UnitType), With<SelectedUnit>>,
) {
    // Use a HashMap to count occurrences of each UnitType.
    let mut counts: HashMap<UnitType, u32> = HashMap::new();
    let mut some_entity: Option<Entity> = None;
    for (entity, unit_type) in q_units.iter() {
        // Capture an entity to use in the event trigger.
        if some_entity.is_none() {
            some_entity = Some(entity);
        }
        // Assuming UnitType implements Copy (or Clone), dereference it for the key.
        *counts.entry(*unit_type).or_insert(0) += 1;
    }

    // Find the unit type with the highest count.
    if let Some((&most_common_unit, _)) = counts.iter().max_by_key(|entry| entry.1) {
        cmds.trigger(UnitAudioEv::new(AudioCmd::Select, most_common_unit));
    }
}

fn single_select(
    trigger: Trigger<SelectSingleUnitEv>,
    mut cmds: Commands,
    q_unit_type: Query<&UnitType>,
) {
    let unit_ent = trigger.event().0;

    let Ok(unit_type) = q_unit_type.get(unit_ent) else {
        return;
    };

    cmds.trigger(UnitAudioEv::new(AudioCmd::Select, *unit_type));
}

fn relocate_cmd(
    _trigger: Trigger<SetUnitDestinationEv>,
    mut cmds: Commands,
    q_units: Query<(Entity, &UnitType), With<SelectedUnit>>,
) {
    // Use a HashMap to count occurrences of each UnitType.
    let mut counts: HashMap<UnitType, u32> = HashMap::new();
    let mut some_entity: Option<Entity> = None;
    for (entity, unit_type) in q_units.iter() {
        // Capture an entity to use in the event trigger.
        if some_entity.is_none() {
            some_entity = Some(entity);
        }
        // Assuming UnitType implements Copy (or Clone), dereference it for the key.
        *counts.entry(*unit_type).or_insert(0) += 1;
    }

    // Find the unit type with the highest count.
    if let Some((&most_common_unit, _)) = counts.iter().max_by_key(|entry| entry.1) {
        cmds.trigger(UnitAudioEv::new(AudioCmd::Relocate, most_common_unit));
    }
}

// fn unit_move_audio(
//     q_destination_add: Query<&SpatialAudioEmitter, Added<pf_comps::Destination>>,
//     mut removed_dest: RemovedComponents<pf_comps::Destination>,
//     q_audio_emitter: Query<&SpatialAudioEmitter>,
//     mut audio_instances: ResMut<Assets<AudioInstance>>,
// ) {
//     for audio_emitter in q_destination_add.iter() {
//         if let Some(instance) = audio_instances.get_mut(&audio_emitter.instances[0]) {
//             instance.resume(AudioTween::default());
//         }
//     }

//     for entity in removed_dest.read() {
//         if let Ok(audio_emitter) = q_audio_emitter.get(entity) {
//             if let Some(instance) = audio_instances.get_mut(&audio_emitter.instances[0]) {
//                 instance.pause(AudioTween::default());
//             }
//         }
//     }
// }
