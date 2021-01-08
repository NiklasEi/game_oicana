use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Map::load_map())
            .add_startup_system(render_map.system())
            .add_startup_system(setup_camera.system());
    }
}

enum Tile {
    Path,
    TowerPlot,
    Tower,
    Castle,
}

pub struct Map {
    height: usize,
    width: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn load_map() -> Self {
        const map_str: &str = "\
            ..........\n\
            ..........\n\
            .......###\n\
            ...#......\n\
            +++++++++*\n\
            ..........\n\
            ..........\n\
            ..........\n\
            ..........\n\
            ..........";

        let mut map = Map {
            height: 0,
            width: 0,
            tiles: vec![],
        };

        for line in map_str.lines() {
            let mut row = vec![];
            for char in line.chars() {
                match char {
                    '.' => row.push(Tile::TowerPlot),
                    '#' => row.push(Tile::Tower),
                    '+' => row.push(Tile::Path),
                    '*' => row.push(Tile::Castle),
                    _ => panic!("unknown map char {}", char),
                }
            }
            map.tiles.push(row);
        }
        // otherwise my map is head down O.o
        map.tiles.reverse();
        map.width = map.tiles.first().unwrap().len();
        map.height = map.tiles.len();

        map
    }
}

fn setup_camera(commands: &mut Commands, map: Res<Map>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            (map.height as f32 / 2.) * 64.,
            (map.width as f32 / 2.) * 64.,
            10.,
        )),
        ..Camera2dBundle::default()
    });
}

fn render_map(
    commands: &mut Commands,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tower_plot_handle: Handle<Texture> = asset_server.load("towerplot64x64.png");
    let tower_handle: Handle<Texture> = asset_server.load("tower64x64.png");
    let path_handle: Handle<Texture> = asset_server.load("path64x64.png");
    let castle_handle: Handle<Texture> = asset_server.load("castle64x64.png");

    for row in 0..map.height {
        for column in 0..map.width {
            let tile = &map.tiles[row][column];
            match tile {
                &Tile::TowerPlot => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(tower_plot_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * 64.,
                            row as f32 * 64.,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
                &Tile::Tower => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(tower_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * 64.,
                            row as f32 * 64.,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
                &Tile::Path => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(path_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * 64.,
                            row as f32 * 64.,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
                &Tile::Castle => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(castle_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * 64.,
                            row as f32 * 64.,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
                _ => (),
            }
        }
    }
}
