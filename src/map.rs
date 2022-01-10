use crate::enemies::Trees;
use crate::loading::TextureAssets;
use crate::AppState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let map = Map::load_map();
        app.insert_resource(map.gather_trees())
            .insert_resource(map)
            .add_system_set(
                SystemSet::on_enter(AppState::Menu)
                    .with_system(render_map.system())
                    .with_system(setup_camera.system()),
            );
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Path,
    Spawn,
    TowerPlot,
    Tower,
    Castle,
    Cloud,
    Empty,
}

#[derive(Default)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Map {
    pub height: usize,
    pub width: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub tile_size: f32,
    pub spawn: Coordinate,
    pub sink: Coordinate,
    pub waypoints: Vec<Coordinate>,
}

#[derive(Component)]
pub struct MapTile {
    pub column: usize,
    pub row: usize,
    pub tile: Tile,
}

impl Map {
    pub fn load_map() -> Self {
        const MAP_STR: &str = "\
            #############\n\
            ########t####\n\
            ###.#.#######\n\
            #a++++0++++q#\n\
            #####+#+#.###\n\
            #t#+++#++.#t#\n\
            ###+.###+++##\n\
            ###+++#.#.+##\n\
            #####++++++##\n\
            ##t#.#.####t#\n\
            #############";

        let mut map = Map {
            height: 0,
            width: 0,
            tiles: vec![],
            tile_size: 64.,
            sink: Coordinate::default(),
            spawn: Coordinate::default(),
            waypoints: vec![],
        };

        let mut preliminary_waypoints = vec![];
        let mut spawn: Point = Default::default();
        let mut sink: Point = Default::default();
        map.height = MAP_STR.lines().count();
        for (row_index, line) in MAP_STR.lines().enumerate() {
            let row_index = map.height - row_index - 1;
            let mut row = vec![];
            for (column_index, char) in line.chars().enumerate() {
                match char {
                    '0' => row.push(Tile::Tower),
                    '.' => row.push(Tile::TowerPlot),
                    '#' => row.push(Tile::Empty),
                    't' => row.push(Tile::Cloud),
                    '+' => {
                        preliminary_waypoints.push(Point {
                            x: column_index,
                            y: row_index,
                        });
                        row.push(Tile::Path)
                    }
                    'a' => {
                        spawn = Point {
                            x: column_index,
                            y: row_index,
                        };
                        map.spawn = Coordinate {
                            x: column_index as f32 * map.tile_size,
                            y: row_index as f32 * map.tile_size,
                        };
                        row.push(Tile::Spawn)
                    }
                    'q' => {
                        sink = Point {
                            x: column_index,
                            y: row_index,
                        };
                        map.sink = Coordinate {
                            x: column_index as f32 * map.tile_size,
                            y: row_index as f32 * map.tile_size,
                        };
                        row.push(Tile::Castle)
                    }
                    _ => panic!("unknown map char {}", char),
                }
            }
            map.tiles.push(row);
        }
        // otherwise my map is head down O.o
        map.tiles.reverse();
        map.width = map.tiles.first().unwrap().len();
        map.create_way_points(preliminary_waypoints, spawn, sink);

        map
    }

    fn create_way_points(&mut self, mut waypoints: Vec<Point>, spawn: Point, sink: Point) {
        let mut last_point = spawn;
        loop {
            let next_point_position = waypoints.iter().position(|point| {
                let length = Vec2::new(
                    last_point.x as f32 - point.x as f32,
                    last_point.y as f32 - point.y as f32,
                )
                .length();
                length > 0.9 && length < 1.1
            });
            if next_point_position.is_none() {
                self.waypoints.push(Coordinate {
                    x: sink.x as f32 * self.tile_size,
                    y: sink.y as f32 * self.tile_size,
                });
                return;
            }
            let next_point_position = next_point_position.unwrap();
            let next_point = waypoints.remove(next_point_position);
            self.waypoints.push(Coordinate {
                x: next_point.x as f32 * self.tile_size,
                y: next_point.y as f32 * self.tile_size,
            });
            last_point = next_point;
        }
    }

    fn gather_trees(&self) -> Trees {
        let mut tree_positions: Vec<Coordinate> = vec![];
        for (row_index, row) in self.tiles.iter().enumerate() {
            for (column_index, tile) in row.iter().enumerate() {
                if tile == &Tile::Cloud {
                    tree_positions.push(Coordinate {
                        x: column_index as f32 * self.tile_size,
                        y: row_index as f32 * self.tile_size,
                    })
                }
            }
        }

        Trees {
            coordinates: tree_positions,
        }
    }
}

fn setup_camera(mut commands: Commands, map: Res<Map>) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation.x = (map.width as f32 / 2. - 0.5) * map.tile_size;
    camera_bundle.transform.translation.y = (map.height as f32 / 2. - 0.5) * map.tile_size;
    commands.spawn_bundle(camera_bundle);
}

fn render_map(mut commands: Commands, map: Res<Map>, texture_assets: Res<TextureAssets>) {
    for row in 0..map.height {
        for column in 0..map.width {
            let tile = &map.tiles[row][column];
            commands
                .spawn_bundle(SpriteBundle {
                    texture: texture_assets.get_handle_for_tile(tile).clone(),
                    transform: Transform::from_translation(Vec3::new(
                        column as f32 * map.tile_size,
                        row as f32 * map.tile_size,
                        0.,
                    )),
                    ..Default::default()
                })
                .insert(MapTile {
                    column,
                    row,
                    tile: tile.clone(),
                });
        }
    }
}
