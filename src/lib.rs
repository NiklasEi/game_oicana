use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::audio::InternalAudioPlugin;
use crate::bullets::BulletPlugin;
use crate::enemies::EnemiesPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::puzzle::PuzzlePlugin;
use crate::towers::TowersPlugin;
use crate::ui::UiPlugin;

mod audio;
mod bullets;
mod enemies;
mod loading;
mod map;
mod menu;
mod puzzle;
mod towers;
mod ui;

pub struct GamePlugin;

pub const MAP_Z: f32 = 0.;
pub const TOWER_Z: f32 = 1.;
pub const PUZZLE_Z: f32 = 2.;
pub const ENEMY_Z: f32 = 3.;
pub const BULLET_Z: f32 = 4.;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Restart,
    InGame,
    Loading,
    Menu,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum OicanaStage {
    EnemyRemoval,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_state(AppState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(ShapePlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(TowersPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(PuzzlePlugin)
            .add_plugin(InternalAudioPlugin);
        app.add_system_set(SystemSet::on_enter(AppState::Restart).with_system(switch_to_game));
    }
}

fn switch_to_game(mut state: ResMut<State<AppState>>) {
    state.set(AppState::InGame).unwrap();
}
