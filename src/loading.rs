use crate::map::Tile;
use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sounds/background.ogg")]
    pub background: Handle<AudioSource>,
    #[asset(path = "sounds/shot.ogg")]
    pub tower_shots: Handle<AudioSource>,
    #[asset(path = "sounds/enemybreach.ogg")]
    pub enemy_breach: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/blank.png")]
    pub blank: Handle<Image>,
    #[asset(path = "textures/towerplot.png")]
    pub tower_plot: Handle<Image>,
    #[asset(path = "textures/tower.png")]
    pub tower: Handle<Image>,
    #[asset(path = "textures/path.png")]
    pub path: Handle<Image>,
    #[asset(path = "textures/castle.png")]
    pub castle: Handle<Image>,
    #[asset(path = "textures/cloud.png")]
    pub cloud: Handle<Image>,
    #[asset(path = "textures/spawn.png")]
    pub spawn: Handle<Image>,
}

impl TextureAssets {
    pub fn get_handle_for_tile(&self, tile: &Tile) -> Handle<Image> {
        match *tile {
            Tile::Empty => self.blank.clone(),
            Tile::TowerPlot => self.tower_plot.clone(),
            Tile::Tower => self.tower.clone(),
            Tile::Path => self.path.clone(),
            Tile::Castle => self.castle.clone(),
            Tile::Cloud => self.cloud.clone(),
            Tile::Spawn => self.spawn.clone(),
        }
    }
}
