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

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::BLACK))
            .add_plugin(MapPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(TowersPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(PuzzlePlugin);
        if cfg!(feature = "native") {
            app.add_plugin(AudioPlugin);
        }
    }
}
