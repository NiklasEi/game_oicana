use crate::enemies::EnemyBreach;
use crate::towers::TowerShot;
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(BackgroundTimer::from_seconds(3. * 60., true))
            .add_startup_system(start_background.system())
            .add_system(tower_shots.system())
            .add_system(enemy_breach.system())
            .add_system(background.system());
    }
}

type BackgroundTimer = Timer;

fn start_background(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("sounds/background.mp3");
    audio.play(music);
}

fn tower_shots(
    mut tower_shot_reader: Local<EventReader<TowerShot>>,
    tower_shot: Res<Events<TowerShot>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if tower_shot_reader.latest(&tower_shot).is_some() {
        let music = asset_server.load("sounds/shot.mp3");
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
        let music = asset_server.load("sounds/background.mp3");
        audio.play(music);
    }
}

fn enemy_breach(
    mut enemy_breach_reader: Local<EventReader<EnemyBreach>>,
    enemy_breach: Res<Events<EnemyBreach>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if enemy_breach_reader.latest(&enemy_breach).is_some() {
        let music = asset_server.load("sounds/enemybreach.mp3");
        audio.play(music);
    }
}
