use bevy::prelude::*;
use rand::seq::IndexedRandom;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    components::units::{Selected, UnitType},
    events::{SelectMultipleUnitEv, SelectSingleUnitEv, SetUnitDestinationEv},
};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyAudio>()
            .add_systems(Startup, load_assets)
            .add_observer(unit_audio)
            .add_observer(multiple_select)
            .add_observer(single_select)
            .add_observer(relocate);
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

#[derive(Debug)]
pub enum AudioCmd {
    Relocate,
    Select,
}

#[derive(Resource, Default)]
pub struct MyAudio {
    pub place_structure: Handle<AudioSource>,
    pub unit_cmds: AudioUnits,
}

#[derive(Default)]
pub struct AudioUnits {
    pub tank_gen_1: AudioUnitCmds,
    pub tank_gen_2: AudioUnitCmds,
}

#[derive(Default)]
pub struct AudioUnitCmds {
    pub relocate: Vec<Handle<AudioSource>>,
    pub select: Vec<Handle<AudioSource>>,
}

fn load_assets(mut my_audio: ResMut<MyAudio>, assets: Res<AssetServer>) {
    my_audio.place_structure = assets.load("audio/place_structure.ogg");

    // tank gen 1 - select
    let folder = "audio/unit_cmds/tank_gen_1/select";
    let handles = load_audio_from_folder(folder, &assets);
    my_audio.unit_cmds.tank_gen_1.select.extend(handles);

    // tank gen 1 - move
    let folder = "audio/unit_cmds/tank_gen_1/move";
    let handles = load_audio_from_folder(folder, &assets);
    my_audio.unit_cmds.tank_gen_1.relocate.extend(handles);

    // tank gen 2 - select
    let folder = "audio/unit_cmds/tank_gen_2/select";
    let handles = load_audio_from_folder(folder, &assets);
    my_audio.unit_cmds.tank_gen_2.select.extend(handles);

    // tank gen 2 - move
    let folder = "audio/unit_cmds/tank_gen_2/move";
    let handles = load_audio_from_folder(folder, &assets);
    my_audio.unit_cmds.tank_gen_2.relocate.extend(handles);
}

fn load_audio_from_folder(folder: &str, assets: &AssetServer) -> Vec<Handle<AudioSource>> {
    // Get the project root (where Cargo.toml is located)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    // Construct the full path to the assets folder.
    let full_folder: PathBuf = [manifest_dir, "assets", folder].iter().collect();

    let mut handles = Vec::new();

    if let Ok(entries) = fs::read_dir(&full_folder) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ogg") {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    // Create an asset path relative to the assets folder.
                    // For example: if folder is "audio/unit_cmds/tank_gen_1/select"
                    // the asset path will be "audio/unit_cmds/tank_gen_1/select/file.ogg".
                    let asset_path = format!("{}/{}", folder, file_name);
                    println!("Loading asset: {}", asset_path);
                    let handle: Handle<AudioSource> = assets.load(&asset_path);
                    handles.push(handle);
                }
            }
        }
    } else {
        error!("Could not read directory: {:?}", full_folder);
    }
    handles
}

// fn load_audio_from_folder(folder: &str, assets: &AssetServer) -> Vec<Handle<AudioSource>> {
//     let mut handles: Vec<Handle<AudioSource>> = Vec::new();
//     if let Ok(entries) = fs::read_dir(folder) {
//         for entry in entries.filter_map(Result::ok) {
//             let path = entry.path();
//             if path.extension().and_then(|s| s.to_str()) == Some("ogg") {
//                 if let Some(relative_path) = path.to_str() {
//                     let relative_path = relative_path
//                         .strip_prefix("../../assets/")
//                         .unwrap_or(relative_path);

//                     // Load the asset using its relative path.
//                     let handle: Handle<AudioSource> = assets.load(relative_path);
//                     handles.push(handle);
//                 }
//             }
//         }
//     } else {
//         error!("Could not read directory: {}", folder);
//     }
//     handles
// }

fn unit_audio(trigger: Trigger<UnitAudioEv>, mut cmds: Commands, my_audio: Res<MyAudio>) {
    let unit_type = &trigger.event().unit;
    let unit_cmd = &trigger.event().cmd;

    // Select the correct list of handles based on unit type and command.
    let handles = match unit_type {
        UnitType::TankGen1 => match unit_cmd {
            AudioCmd::Relocate => &my_audio.unit_cmds.tank_gen_1.relocate,
            AudioCmd::Select => &my_audio.unit_cmds.tank_gen_1.select,
        },
        UnitType::TankGen2 => match unit_cmd {
            AudioCmd::Relocate => &my_audio.unit_cmds.tank_gen_2.relocate,
            AudioCmd::Select => &my_audio.unit_cmds.tank_gen_2.select,
        },
        // TODO: Temporary
        UnitType::Rifleman => match unit_cmd {
            AudioCmd::Relocate => &my_audio.unit_cmds.tank_gen_2.relocate,
            AudioCmd::Select => &my_audio.unit_cmds.tank_gen_2.select,
        },
    };

    if let Some(handle) = handles.choose(&mut rand::rng()) {
        let audio_cmd = AudioPlayer::new(handle.clone());
        cmds.spawn(audio_cmd);
    } else {
        eprintln!("Audio Handle Missing");
    }
}

fn multiple_select(
    _trigger: Trigger<SelectMultipleUnitEv>,
    mut cmds: Commands,
    q_units: Query<(Entity, &UnitType), With<Selected>>,
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

fn relocate(
    _trigger: Trigger<SetUnitDestinationEv>,
    mut cmds: Commands,
    q_units: Query<(Entity, &UnitType), With<Selected>>,
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
