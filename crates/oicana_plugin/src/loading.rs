use crate::map::Tile;
use crate::{AppState, STAGE};
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, AppState::Loading, start_loading.system())
            .on_state_update(STAGE, AppState::Loading, check_state.system());
    }
}

pub struct LoadingState {
    sound: Vec<HandleUntyped>,
    textures: Vec<HandleUntyped>,
}

pub struct AudioAssets {
    pub background: Handle<AudioSource>,
    pub tower_shots: Handle<AudioSource>,
    pub enemy_breach: Handle<AudioSource>,
}

pub struct TextureAssets {
    pub empty_handle: Handle<Texture>,
    pub tower_plot_handle: Handle<Texture>,
    pub tower_handle: Handle<Texture>,
    pub path_handle: Handle<Texture>,
    pub castle_handle: Handle<Texture>,
    pub cloud_handle: Handle<Texture>,
    pub spawn_handle: Handle<Texture>,
}

impl TextureAssets {
    pub fn get_handle_for_tile(&self, tile: &Tile) -> Handle<Texture> {
        match tile {
            &Tile::Empty => self.empty_handle.clone(),
            &Tile::TowerPlot => self.tower_plot_handle.clone(),
            &Tile::Tower => self.tower_handle.clone(),
            &Tile::Path => self.path_handle.clone(),
            &Tile::Castle => self.castle_handle.clone(),
            &Tile::Cloud => self.cloud_handle.clone(),
            &Tile::Spawn => self.spawn_handle.clone(),
        }
    }
}

fn start_loading(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let sound: Vec<HandleUntyped> = asset_server
        .load_folder("sounds")
        .expect("Failed to load sounds");
    let textures: Vec<HandleUntyped> = asset_server
        .load_folder("textures")
        .expect("Failed to load textures");

    commands.insert_resource(LoadingState { sound, textures });
}

fn check_state(
    mut state: ResMut<State<AppState>>,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.sound.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(AudioAssets {
        background: asset_server.get_handle("sounds/background.mp3"),
        tower_shots: asset_server.get_handle("sounds/shot.mp3"),
        enemy_breach: asset_server.get_handle("sounds/enemybreach.mp3"),
    });

    commands.insert_resource(TextureAssets {
        empty_handle: asset_server.get_handle("textures/blank64x64.png"),
        tower_plot_handle: asset_server.get_handle("textures/towerplot64x64.png"),
        tower_handle: asset_server.get_handle("textures/tower64x64.png"),
        path_handle: asset_server.get_handle("textures/path64x64.png"),
        castle_handle: asset_server.get_handle("textures/castle64x64.png"),
        cloud_handle: asset_server.get_handle("textures/cloud64x64.png"),
        spawn_handle: asset_server.get_handle("textures/spawn.png"),
    });

    state.set_next(AppState::InGame).unwrap();
}
