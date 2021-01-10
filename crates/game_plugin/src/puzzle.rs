use crate::enemies::{
    build_circle_path, build_quadratic_path, build_triangle_path, EnemyColor, EnemyForm,
};
use crate::map::{Coordinate, Map, Tile};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{FillOptions, LineJoin, StrokeOptions};

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Puzzles { new_towers: vec![] })
            .add_startup_system(set_tower_puzzles.system());
    }
}

pub struct PuzzleSlot {
    piece: Piece,
}

pub struct Puzzles {
    new_towers: Vec<Puzzle>,
}

pub struct Puzzle {
    coordinate: Coordinate,
    pieces: [Piece; 4],
}

#[derive(Debug, Clone)]
pub struct Piece {
    color: EnemyColor,
    form: EnemyForm,
}

fn set_tower_puzzles(
    mut meshes: ResMut<Assets<Mesh>>,
    commands: &mut Commands,
    mut puzzles: ResMut<Puzzles>,
    map: Res<Map>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut new_tower_positions: Vec<Coordinate> = vec![];
    for (row_index, row) in map.tiles.iter().enumerate() {
        for (column_index, tile) in row.iter().enumerate() {
            if tile == &Tile::TowerPlot {
                new_tower_positions.push(Coordinate {
                    x: column_index as f32 * map.tile_size,
                    y: row_index as f32 * map.tile_size,
                })
            }
        }
    }

    for coordinate in new_tower_positions {
        let puzzle = Puzzle {
            coordinate: coordinate.clone(),
            pieces: [
                Piece {
                    color: EnemyColor::Red,
                    form: EnemyForm::Quadratic,
                },
                Piece {
                    color: EnemyColor::Blue,
                    form: EnemyForm::Triangle,
                },
                Piece {
                    color: EnemyColor::Blue,
                    form: EnemyForm::Circle,
                },
                Piece {
                    color: EnemyColor::Red,
                    form: EnemyForm::Triangle,
                },
            ],
        };
        for (index, piece) in puzzle.pieces.iter().enumerate() {
            let path = match piece.form {
                EnemyForm::Circle => build_circle_path(),
                EnemyForm::Triangle => build_triangle_path(),
                EnemyForm::Quadratic => build_quadratic_path(),
            };
            let coordinate = match index {
                0 => Coordinate {
                    x: coordinate.x - 16.,
                    y: coordinate.y - 16.,
                },
                1 => Coordinate {
                    x: coordinate.x + 16.,
                    y: coordinate.y - 16.,
                },
                2 => Coordinate {
                    x: coordinate.x + 16.,
                    y: coordinate.y + 16.,
                },
                _ => Coordinate {
                    x: coordinate.x - 16.,
                    y: coordinate.y + 16.,
                },
            };

            commands
                .spawn(
                    path.stroke(
                        materials.add(piece.color.to_color().into()),
                        &mut meshes,
                        Vec3::new(coordinate.x, coordinate.y, 0.),
                        &StrokeOptions::default()
                            .with_line_width(2.)
                            .with_line_join(LineJoin::Round),
                    ),
                )
                .with(PuzzleSlot {
                    piece: piece.clone(),
                });
        }

        puzzles.new_towers.push(puzzle);
    }
}
