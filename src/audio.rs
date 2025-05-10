use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioApp, AudioControl, AudioPlugin, AudioSource, prelude::Volume};

use crate::gameplay::GameState;

// This plugin is responsible to control the game audio
pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin)
        .add_audio_channel::<BGM>()
        .add_audio_channel::<SFX>()
        .add_systems(Startup, (load_audio, loop_bgm).chain());
}
#[derive(Resource)]
struct AudioState {
    battle_handle: Handle<AudioSource>,
}

#[derive(Resource, Component, Default, Clone)]
struct BGM;
#[derive(Resource, Component, Default, Clone)]
struct SFX;

fn load_audio(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    // let battle_handle = asset_server.load(".audio/music/battle_music_v1.ogg");
    let battle_handle = asset_server.load("audio/music/bgm.ogg");
    commands.insert_resource(AudioState { battle_handle });
}

fn loop_bgm(audio_state: Res<AudioState>, audio: Res<Audio>) {
    audio
        .play(audio_state.battle_handle.clone())
        .loop_from(22.6)
        .with_volume(Volume::Amplitude(0.2));
}

fn rewind_sfx(trigger: Trigger<OnEnter<GameState>>) {}
