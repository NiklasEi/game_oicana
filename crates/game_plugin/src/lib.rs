mod bullets;
mod enemies;
mod map;
mod towers;

use crate::bullets::BulletPlugin;
use crate::enemies::EnemiesPlugin;
use crate::map::MapPlugin;
use crate::towers::TowersPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::LIME_GREEN))
            .add_plugin(MapPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(TowersPlugin)
            .add_plugin(BulletPlugin);
    }
}
