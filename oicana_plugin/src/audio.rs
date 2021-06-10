use crate::enemies::EnemyBreach;
use crate::loading::AudioAssets;
use crate::towers::TowerShot;
use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(start_audio.system()))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(tower_shots.system())
                    .with_system(enemy_breach.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(stop_audio.system()));
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
