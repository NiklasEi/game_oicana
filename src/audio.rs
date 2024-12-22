use crate::enemies::EnemyBreach;
use crate::loading::AudioAssets;
use crate::towers::TowerShot;
use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioControl, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_audio_channel::<BackgroundAudio>()
            .add_systems(OnEnter(AppState::Menu), start_audio)
            .add_systems(
                Update,
                (tower_shots, enemy_breach).run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), stop_audio);
    }
}

#[derive(Resource)]
struct BackgroundAudio;

fn start_audio(
    audio_assets: Res<AudioAssets>,
    background_channel: Res<AudioChannel<BackgroundAudio>>,
    audio: Res<Audio>,
) {
    audio.set_volume(0.15);
    background_channel.set_volume(0.15);
    background_channel
        .play(audio_assets.background.clone())
        .looped();
}

fn stop_audio(audio: Res<Audio>) {
    audio.stop();
}

fn tower_shots(
    audio_assets: Res<AudioAssets>,
    mut tower_shot_reader: EventReader<TowerShot>,
    audio: Res<Audio>,
) {
    if tower_shot_reader.iter().last().is_some() {
        audio.play(audio_assets.tower_shots.clone());
    }
}

fn enemy_breach(
    audio_assets: Res<AudioAssets>,
    mut enemy_breach_reader: EventReader<EnemyBreach>,
    audio: Res<Audio>,
) {
    if enemy_breach_reader.iter().last().is_some() {
        audio.play(audio_assets.enemy_breach.clone());
    }
}
