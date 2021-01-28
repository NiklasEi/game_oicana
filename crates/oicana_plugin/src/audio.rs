use crate::enemies::EnemyBreach;
use crate::towers::TowerShot;
use crate::{AppState, STAGE};
use bevy::prelude::*;
use bevy_improved_audio::{Audio, AudioPlugin, AudioSource};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .on_state_enter(STAGE, AppState::InGame, start_audio.system())
            .on_state_update(STAGE, AppState::InGame, tower_shots.system())
            .on_state_update(STAGE, AppState::InGame, enemy_breach.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_audio.system());
    }
}

fn start_audio(asset_server: Res<AssetServer>, mut audio: ResMut<Audio>) {
    let music: Handle<AudioSource> = asset_server.load("sounds/background.mp3");
    audio.play(music);
}

fn tower_shots(
    asset_server: Res<AssetServer>,
    mut tower_shot_reader: Local<EventReader<TowerShot>>,
    tower_shot: Res<Events<TowerShot>>,
    mut audio: ResMut<Audio>,
) {
    if tower_shot_reader.latest(&tower_shot).is_some() {
        let music = asset_server.load("sounds/shot.mp3");
        audio.play(music);
    }
}

fn enemy_breach(
    asset_server: Res<AssetServer>,
    mut enemy_breach_reader: Local<EventReader<EnemyBreach>>,
    enemy_breach: Res<Events<EnemyBreach>>,
    mut audio: ResMut<Audio>,
) {
    if enemy_breach_reader.latest(&enemy_breach).is_some() {
        let music = asset_server.load("sounds/enemybreach.mp3");
        audio.play(music);
    }
}

fn break_down_audio(mut audio: ResMut<Audio>) {
    // if let Some(background) = &audio.background_handle.clone() {
    //     audio.manager.remove_arrangement(background).unwrap();
    // }
}
