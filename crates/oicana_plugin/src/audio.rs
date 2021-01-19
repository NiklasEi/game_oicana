use crate::enemies::EnemyBreach;
use crate::towers::TowerShot;
use crate::{AppState, STAGE};
use bevy::prelude::{
    AppBuilder, AssetServer, EventReader, Events, Handle, IntoSystem, Local, Plugin, Res, ResMut,
    Time, Timer,
};
use bevy_improved_audio::{Audio, AudioPlugin, AudioSource};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_resource(BackgroundTimer::from_seconds(3. * 60., true))
            .on_state_enter(STAGE, AppState::InGame, start_background.system())
            .on_state_update(STAGE, AppState::InGame, tower_shots.system())
            .on_state_update(STAGE, AppState::InGame, enemy_breach.system())
            .on_state_update(STAGE, AppState::InGame, background.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_audio.system());
    }
}

type BackgroundTimer = Timer;

fn start_background(asset_server: Res<AssetServer>, audio: Res<Audio<AudioSource>>) {
    let music: Handle<AudioSource> = asset_server.load("sounds/background.ogg");
    audio.play_in_channel(music, "background".to_owned());
}

fn tower_shots(
    mut tower_shot_reader: Local<EventReader<TowerShot>>,
    tower_shot: Res<Events<TowerShot>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if tower_shot_reader.latest(&tower_shot).is_some() {
        let music = asset_server.load("sounds/shot.ogg");
        audio.play(music);
    }
}

fn background(
    mut timer: ResMut<BackgroundTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    timer.tick(time.delta_seconds());
    if timer.just_finished() {
        let music = asset_server.load("sounds/background.ogg");
        audio.play_in_channel(music, "background".to_owned());
    }
}

fn enemy_breach(
    mut enemy_breach_reader: Local<EventReader<EnemyBreach>>,
    enemy_breach: Res<Events<EnemyBreach>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if enemy_breach_reader.latest(&enemy_breach).is_some() {
        let music: Handle<AudioSource> = asset_server.load("sounds/enemybreach.ogg");
        audio.play(music);
    }
}

fn break_down_audio(audio: Res<Audio>) {
    audio.drop_channel("background".to_owned());
}
