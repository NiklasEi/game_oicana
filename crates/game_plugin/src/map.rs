use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Map::load_map())
            .add_startup_system(render_map.system())
            .add_startup_system(setup_camera.system());
    }
}

#[derive(Debug, PartialEq)]
pub enum Tile {
    Path,
    TowerPlot,
    Tower,
    Castle,
    Empty,
}

#[derive(Default)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug)]
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

impl Map {
    pub fn load_map() -> Self {
        const MAP_STR: &str = "\
            #############\n\
            #############\n\
            ###...#######\n\
            ##a+++0++++q#\n\
            #####+#+#####\n\
            ###+++#++####\n\
            ###+0###+++##\n\
            ###+++#.#0+##\n\
            #####++++++##\n\
            #############\n\
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
                        row.push(Tile::Path)
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
}

fn setup_camera(commands: &mut Commands, map: Res<Map>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            (map.width as f32 / 2. - 0.5) * map.tile_size,
            (map.height as f32 / 2. - 0.5) * map.tile_size,
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
    let blank_handle: Handle<Texture> = asset_server.load("blank64x64.png");
    let tower_plot_handle: Handle<Texture> = asset_server.load("towerplot64x64.png");
    let tower_handle: Handle<Texture> = asset_server.load("tower64x64.png");
    let path_handle: Handle<Texture> = asset_server.load("path64x64.png");
    let castle_handle: Handle<Texture> = asset_server.load("castle64x64.png");

    for row in 0..map.height {
        for column in 0..map.width {
            let tile = &map.tiles[row][column];
            match tile {
                &Tile::Empty => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(blank_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * map.tile_size,
                            row as f32 * map.tile_size,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
                &Tile::TowerPlot => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(tower_plot_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * map.tile_size,
                            row as f32 * map.tile_size,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
                &Tile::Tower => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(tower_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * map.tile_size,
                            row as f32 * map.tile_size,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
                &Tile::Path => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(path_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * map.tile_size,
                            row as f32 * map.tile_size,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
                &Tile::Castle => {
                    commands.spawn(SpriteBundle {
                        material: materials.add(castle_handle.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * map.tile_size,
                            row as f32 * map.tile_size,
                            0.,
                        )),
                        ..Default::default()
                    });
                }
            }
        }
    }
}
