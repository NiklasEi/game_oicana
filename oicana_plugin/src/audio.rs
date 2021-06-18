use crate::enemies::EnemyBreach;
use crate::loading::AudioAssets;
use crate::towers::TowerShot;
use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(start_audio.system()))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(tower_shots.system())
                    .with_system(enemy_breach.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(stop_audio.system()));
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    let background_channel = AudioChannel::new("background".to_owned());
    audio.set_volume(0.15);
    audio.set_volume_in_channel(0.15, &background_channel);
    audio.play_looped_in_channel(audio_assets.background.clone(), &background_channel);
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
