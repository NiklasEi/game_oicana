#[cfg(feature = "native")]
mod audio;
mod bullets;
mod enemies;
mod map;
mod puzzle;
mod towers;
mod ui;

#[cfg(feature = "native")]
use crate::audio::AudioPlugin;
use crate::bullets::BulletPlugin;
use crate::enemies::EnemiesPlugin;
use crate::map::MapPlugin;
use crate::puzzle::PuzzlePlugin;
use crate::towers::TowersPlugin;
use crate::ui::UiPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

const STAGE: &str = "stage";

#[derive(Clone)]
pub enum AppState {
    Menu,
    InGame,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::BLACK))
            .add_resource(State::new(AppState::InGame))
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
            .add_plugin(MapPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(TowersPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(PuzzlePlugin);
        #[cfg(feature = "native")]
        app.add_plugin(AudioPlugin);
        app.on_state_enter(STAGE, AppState::Menu, switch_to_game.system());
    }
}

fn switch_to_game(mut state: ResMut<State<AppState>>) {
    state.set_next(AppState::InGame).unwrap();
}
