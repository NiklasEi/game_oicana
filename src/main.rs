// disable console opening on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowResolution;
use oicana::GamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.),
                title: "Oicana".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
