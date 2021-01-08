mod map;

use crate::map::MapPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::ALICE_BLUE))
            .add_plugin(MapPlugin);
    }
}
