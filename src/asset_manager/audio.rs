use bevy::prelude::*;
use bevy_kira_audio::*;
use rand::seq::IndexedRandom;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

use crate::events::*;
use crate::resources::DbgOptions;
use crate::units::components::*;
use bevy_rts_pathfinding::components as pf_comps;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyAudio>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, unit_move_audio)
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

#[derive(Debug)]
pub enum AudioCmd {
    Relocate,
    Select,
    Ready,
}

#[derive(Resource, Default)]
pub struct MyAudio {
    pub place_structure: Handle<bevy::prelude::AudioSource>,
    pub unit_cmds: AudioUnits,
    pub sfx: Sfx,
}

#[derive(Default)]
pub struct AudioUnits {
    pub tank_gen_1: AudioUnitCmds,
    pub tank_gen_2: AudioUnitCmds,
}

#[derive(Default)]
pub struct AudioUnitCmds {
    pub relocate: Vec<Handle<bevy::prelude::AudioSource>>,
    pub select: Vec<Handle<bevy::prelude::AudioSource>>,
    pub ready: Vec<Handle<bevy::prelude::AudioSource>>,
}

#[derive(Default)]
pub struct Sfx {
    pub moving_rifleman: SfxOptions,
    pub moving_tank_gen_1: SfxOptions,
    pub moving_tank_gen_2: SfxOptions,
}

#[derive(Default)]
pub struct SfxOptions {
    pub source: Handle<bevy_kira_audio::AudioSource>,
    // pub instance: Handle<bevy_kira_audio::AudioInstance>,
}

fn load_assets(
    // audio: Res<bevy_kira_audio::Audio>,
    mut my_audio: ResMut<MyAudio>,
    assets: Res<AssetServer>,
    dbg: Res<DbgOptions>,
) {
    my_audio.place_structure = assets.load("audio/place_structure.ogg");

    // unit moving (spatial)
    // let handle = assets.load("audio/sfx/rifleman/moving.ogg");
    // my_audio.sfx.moving_rifleman.source = handle.clone();
    // my_audio.sfx.moving_rifleman.instance = audio.play(handle).looped().paused().handle();

    // let handle = assets.load("audio/sfx/tank_gen_1/moving.ogg");
    // my_audio.sfx.moving_tank_gen_1.source = handle.clone();
    // my_audio.sfx.moving_tank_gen_1.instance = audio.play(handle).looped().paused().handle();

    // let handle = assets.load("audio/sfx/tank_gen_2/moving.ogg");
    // my_audio.sfx.moving_tank_gen_2.source = handle.clone();
    // my_audio.sfx.moving_tank_gen_2.instance = audio.play(handle).looped().paused().handle();

    // tank gen 1 - cmd select
    let folder = "audio/unit_cmds/tank_gen_1/select";
    let handles = load_audio_from_folder(folder, &assets, &dbg);
    my_audio.unit_cmds.tank_gen_1.select.extend(handles);

    // tank gen 1 - cmd move
    let folder = "audio/unit_cmds/tank_gen_1/move";
    let handles = load_audio_from_folder(folder, &assets, &dbg);
    my_audio.unit_cmds.tank_gen_1.relocate.extend(handles);

    // tank gen 1 - cmd ready
    let folder = "audio/unit_cmds/tank_gen_1/ready";
    let handles = load_audio_from_folder(folder, &assets, &dbg);
    my_audio.unit_cmds.tank_gen_1.ready.extend(handles);

    // tank gen 2 - cmd select
    let folder = "audio/unit_cmds/tank_gen_2/select";
    let handles = load_audio_from_folder(folder, &assets, &dbg);
    my_audio.unit_cmds.tank_gen_2.select.extend(handles);

    // tank gen 2 - cmd move
    let folder = "audio/unit_cmds/tank_gen_2/move";
    let handles = load_audio_from_folder(folder, &assets, &dbg);
    my_audio.unit_cmds.tank_gen_2.relocate.extend(handles);

    // tank gen 2 - cmd ready
    let folder = "audio/unit_cmds/tank_gen_2/ready";
    let handles = load_audio_from_folder(folder, &assets, &dbg);
    my_audio.unit_cmds.tank_gen_2.ready.extend(handles);
}

fn load_audio_from_folder(
    folder: &str,
    assets: &AssetServer,
    dbg: &DbgOptions,
) -> Vec<Handle<bevy::prelude::AudioSource>> {
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
                    dbg.print(&format!("Loading asset: {}", asset_path));

                    let handle: Handle<bevy::prelude::AudioSource> = assets.load(&asset_path);
                    handles.push(handle);
                }
            }
        }
    } else {
        error!("Could not read directory: {:?}", full_folder);
    }
    handles
}

fn unit_audio(trigger: Trigger<UnitAudioEv>, mut cmds: Commands, my_audio: Res<MyAudio>) {
    let unit_type = &trigger.event().unit;
    let unit_cmd = &trigger.event().cmd;

    // Select the correct list of handles based on unit type and command.
    let handles = match unit_type {
        UnitType::TankGen1 => match unit_cmd {
            AudioCmd::Relocate => &my_audio.unit_cmds.tank_gen_1.relocate,
            AudioCmd::Select => &my_audio.unit_cmds.tank_gen_1.select,
            AudioCmd::Ready => &my_audio.unit_cmds.tank_gen_1.ready,
        },
        UnitType::TankGen2 => match unit_cmd {
            AudioCmd::Relocate => &my_audio.unit_cmds.tank_gen_2.relocate,
            AudioCmd::Select => &my_audio.unit_cmds.tank_gen_2.select,
            AudioCmd::Ready => &my_audio.unit_cmds.tank_gen_2.ready,
        },
        // TODO: Temporary
        UnitType::Artillery => match unit_cmd {
            AudioCmd::Relocate => &my_audio.unit_cmds.tank_gen_2.relocate,
            AudioCmd::Select => &my_audio.unit_cmds.tank_gen_2.select,
            AudioCmd::Ready => &my_audio.unit_cmds.tank_gen_2.ready,
        },
        // TODO: Temporary
        UnitType::Rifleman => match unit_cmd {
            AudioCmd::Relocate => &my_audio.unit_cmds.tank_gen_2.relocate,
            AudioCmd::Select => &my_audio.unit_cmds.tank_gen_2.select,
            AudioCmd::Ready => &my_audio.unit_cmds.tank_gen_2.ready,
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

fn unit_move_audio(
    q_destination_add: Query<&SpatialAudioEmitter, Added<pf_comps::Destination>>,
    mut removed_dest: RemovedComponents<pf_comps::Destination>,
    q_audio_emitter: Query<&SpatialAudioEmitter>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for audio_emitter in q_destination_add.iter() {
        if let Some(instance) = audio_instances.get_mut(&audio_emitter.instances[0]) {
            instance.resume(AudioTween::default());
        }
    }

    for entity in removed_dest.read() {
        if let Ok(audio_emitter) = q_audio_emitter.get(entity) {
            if let Some(instance) = audio_instances.get_mut(&audio_emitter.instances[0]) {
                instance.pause(AudioTween::default());
            }
        }
    }
}
