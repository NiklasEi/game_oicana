// disable console opening on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use oicana_plugin::GamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Oicana".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
