use crate::enemies::EnemyBreach;
use crate::loading::AudioAssets;
use crate::towers::TowerShot;
use crate::{AppState, STAGE};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .on_state_enter(STAGE, AppState::InGame, start_audio.system())
            .on_state_update(STAGE, AppState::InGame, tower_shots.system())
            .on_state_update(STAGE, AppState::InGame, enemy_breach.system())
            .on_state_exit(STAGE, AppState::InGame, stop_audio.system());
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume(0.15);
    audio.play_looped(audio_assets.background.clone());
}

fn stop_audio(audio: Res<Audio>) {
    audio.stop();
}

fn tower_shots(
    audio_assets: Res<AudioAssets>,
    mut tower_shot_reader: Local<EventReader<TowerShot>>,
    tower_shot: Res<Events<TowerShot>>,
    audio: Res<Audio>,
) {
    if tower_shot_reader.latest(&tower_shot).is_some() {
        audio.play(audio_assets.tower_shots.clone());
    }
}

fn enemy_breach(
    audio_assets: Res<AudioAssets>,
    mut enemy_breach_reader: Local<EventReader<EnemyBreach>>,
    enemy_breach: Res<Events<EnemyBreach>>,
    audio: Res<Audio>,
) {
    if enemy_breach_reader.latest(&enemy_breach).is_some() {
        audio.play(audio_assets.enemy_breach.clone());
    }
}
