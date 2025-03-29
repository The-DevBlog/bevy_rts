use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyAudio>();
    }
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

fn add_assets(mut my_audio: ResMut<MyAudio>, assets: Res<AssetServer>) {
    my_audio.place_structure = assets.load("audio/place_structure.ogg");
}
