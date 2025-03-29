use std::fs;

use bevy::prelude::*;

use crate::components::units::UnitType;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyAudio>()
            .add_systems(Startup, load_assets)
            .add_observer(unit_audio);
    }
}

#[derive(Event)]
// pub struct UnitAudioEv(pub UnitAudioOptions);
pub struct UnitAudioEv {
    pub cmd: AudioCmd,
    pub unit: UnitType,
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
    let mut handles: Vec<Handle<AudioSource>> = Vec::new();
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ogg") {
                if let Some(relative_path) = path.to_str() {
                    // Load the asset using its relative path.
                    let handle: Handle<AudioSource> = assets.load(relative_path);
                    handles.push(handle);
                }
            }
        }
    } else {
        error!("Could not read directory: {}", folder);
    }
    handles
}

fn unit_audio(trigger: Trigger<UnitAudioEv>, mut cmds: Commands, my_audio: Res<MyAudio>) {
    // let mut bundle = AudioBundle::default();
    // bundle.settings.mode = PlaybackMode::Despawn;

    // let audio = AudioS

    let unit_type = trigger.event().unit;
    let audio_cmd = trigger.event().cmd;

    let audio = match unit_type {
        UnitType::TankGen1 => {
            AudioPlayer::new(my_audio.unit_cmds.tank_gen_1, audio_cmd, &mut cmds);
        }
        UnitType::TankGen2 => {
            AudioPlayer::new(my_audio.unit_cmds.tank_gen_2, audio_cmd, &mut cmds);
        }
    }
    // }
}
