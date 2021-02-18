mod audio;
mod bullets;
mod enemies;
mod loading;
mod map;
mod menu;
mod puzzle;
mod towers;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::bullets::BulletPlugin;
use crate::enemies::EnemiesPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::puzzle::PuzzlePlugin;
use crate::towers::TowersPlugin;
use crate::ui::UiPlugin;
use crate::menu::MenuPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

const STAGE: &str = "oicana_stage";

#[derive(Clone)]
pub enum AppState {
    Restart,
    InGame,
    Loading,
    Menu,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::BLACK))
            .add_resource(State::new(AppState::Loading))
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
            .add_plugin(MenuPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(TowersPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(PuzzlePlugin);
        app.add_plugin(InternalAudioPlugin);
        app.on_state_enter(STAGE, AppState::Restart, switch_to_game.system());
    }
}

fn switch_to_game(mut state: ResMut<State<AppState>>) {
    state.set_next(AppState::InGame).unwrap();
}
