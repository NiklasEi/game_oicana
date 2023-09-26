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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    Restart,
    InGame,
    #[default]
    Loading,
    Menu,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_state::<AppState>()
            .add_plugins((
                LoadingPlugin,
                ShapePlugin,
                MenuPlugin,
                MapPlugin,
                EnemiesPlugin,
                TowersPlugin,
                BulletPlugin,
                UiPlugin,
                PuzzlePlugin,
                InternalAudioPlugin,
            ));
        app.add_systems(OnEnter(AppState::Restart), switch_to_game);
    }
}

fn switch_to_game(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
}
