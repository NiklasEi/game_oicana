pub struct AssetPaths {
    pub fira_sans: &'static str,
    pub sound_background: &'static str,
    pub sound_tower_shots: &'static str,
    pub sound_enemy_breach: &'static str,
    pub texture_empty: &'static str,
    pub texture_tower_plot: &'static str,
    pub texture_tower: &'static str,
    pub texture_path: &'static str,
    pub texture_castle: &'static str,
    pub texture_cloud: &'static str,
    pub texture_spawn: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    fira_sans: "fonts/FiraSans-Bold.ttf",
    sound_background: "sounds/background.ogg",
    sound_tower_shots: "sounds/shot.ogg",
    sound_enemy_breach: "sounds/enemybreach.ogg",
    texture_empty: "textures/blank64x64.png",
    texture_tower_plot: "textures/towerplot64x64.png",
    texture_tower: "textures/tower64x64.png",
    texture_path: "textures/path64x64.png",
    texture_castle: "textures/castle64x64.png",
    texture_cloud: "textures/cloud64x64.png",
    texture_spawn: "textures/spawn.png",
};
