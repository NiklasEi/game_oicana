use crate::enemies::{
    build_circle_path, build_triangle_path, Enemy, EnemyColor, EnemyForm, Tameable,
};
use crate::map::{Coordinate, Map, Tile};
use crate::{AppState, STAGE};
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::prelude::{FillOptions, LineJoin, PathBuilder, StrokeOptions};
use bevy_prototype_lyon::shapes;
use bevy_prototype_lyon::utils::TessellationMode;
use rand::random;
use std::f32::consts::PI;

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PuzzleIdFactory::default())
            .add_resource(PickSource {
                cursor_offset: Vec2::new(17., -19.),
                ..Default::default()
            })
            .add_resource(CurrentPiece {
                entity: None,
                piece: None,
            })
            .add_event::<CompletePuzzle>()
            .add_resource(Puzzles { towers: vec![] })
            .on_state_enter(STAGE, AppState::InGame, set_tower_puzzles.system())
            .on_state_update(STAGE, AppState::InGame, pick_up_piece.system())
            .on_state_update(STAGE, AppState::InGame, update_picked_up_piece.system())
            .on_state_update(STAGE, AppState::InGame, update_puzzle_slots.system())
            .on_state_update(STAGE, AppState::InGame, update_puzzle.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_puzzles.system());
    }
}

#[derive(Debug)]
pub struct CompletePuzzle {
    pub coordinate: Coordinate,
    puzzle_id: usize,
}

#[derive(Default)]
struct PuzzleIdFactory {
    next_id: usize,
}

impl PuzzleIdFactory {
    pub fn get_next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id - 1
    }
}

#[derive(Clone)]
pub struct PuzzleSlot {
    piece: Piece,
    filled: bool,
    puzzle_id: usize,
}

pub struct CurrentPiece {
    pub entity: Option<Entity>,
    pub piece: Option<Piece>,
}

#[derive(Default)]
pub struct PickSource {
    pub cursor_events: EventReader<CursorMoved>,
    pub last_cursor_pos: Vec2,
    pub cursor_offset: Vec2,
}

pub struct Puzzles {
    towers: Vec<Puzzle>,
}

pub struct Puzzle {
    id: usize,
    coordinate: Coordinate,
    pieces: [Piece; 4],
    filled: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    color: EnemyColor,
    form: EnemyForm,
}

struct ToFill;

fn set_tower_puzzles(
    commands: &mut Commands,
    mut puzzles: ResMut<Puzzles>,
    map: Res<Map>,
    mut puzzle_ids: ResMut<PuzzleIdFactory>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut tower_positions: Vec<Coordinate> = vec![];
    for (row_index, row) in map.tiles.iter().enumerate() {
        for (column_index, tile) in row.iter().enumerate() {
            if tile == &Tile::TowerPlot || tile == &Tile::Tower {
                tower_positions.push(Coordinate {
                    x: column_index as f32 * map.tile_size,
                    y: row_index as f32 * map.tile_size,
                })
            }
        }
    }

    for coordinate in tower_positions {
        let id = puzzle_ids.get_next_id();
        let puzzle = spawn_puzzle(id, coordinate, commands, &mut materials);

        puzzles.towers.push(puzzle);
    }
}

fn spawn_puzzle(
    id: usize,
    coordinate: Coordinate,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Puzzle {
    let puzzle = Puzzle {
        coordinate: coordinate.clone(),
        filled: 0,
        id,
        pieces: [
            Piece {
                color: random(),
                form: random(),
            },
            Piece {
                color: random(),
                form: random(),
            },
            Piece {
                color: random(),
                form: random(),
            },
            Piece {
                color: random(),
                form: random(),
            },
        ],
    };
    for (index, piece) in puzzle.pieces.iter().enumerate() {
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

        let bundle: ShapeBundle = match piece.form {
            EnemyForm::Circle => GeometryBuilder::build_as(
                &build_circle_path(),
                materials.add(ColorMaterial::color(piece.color.to_color())),
                TessellationMode::Stroke(
                    StrokeOptions::default()
                        .with_line_width(2.)
                        .with_line_join(LineJoin::Round),
                ),
                Transform::from_translation(Vec3::new(coordinate.x, coordinate.y, 0.)),
            ),
            EnemyForm::Triangle => GeometryBuilder::build_as(
                &build_triangle_path(),
                materials.add(ColorMaterial::color(piece.color.to_color())),
                TessellationMode::Stroke(
                    StrokeOptions::default()
                        .with_line_width(2.)
                        .with_line_join(LineJoin::Round),
                ),
                Transform::from_translation(Vec3::new(coordinate.x, coordinate.y, 0.)),
            ),
            EnemyForm::Quadratic => {
                let rectangle = shapes::Rectangle {
                    width: 18.0,
                    height: 18.0,
                    ..shapes::Rectangle::default()
                };
                let mut builder = GeometryBuilder::new();
                builder.add(&rectangle);
                builder.build(
                    materials.add(ColorMaterial::color(piece.color.to_color())),
                    TessellationMode::Stroke(
                        StrokeOptions::default()
                            .with_line_width(2.)
                            .with_line_join(LineJoin::Round),
                    ),
                    Transform::from_translation(Vec3::new(coordinate.x, coordinate.y, 0.)),
                )
            }
        };
        commands.spawn(bundle).with(PuzzleSlot {
            piece: piece.clone(),
            filled: false,
            puzzle_id: id,
        });
    }
    puzzle
}

fn update_puzzle_slots(
    commands: &mut Commands,
    mut puzzles: ResMut<Puzzles>,
    query: Query<(Entity, &Transform, &PuzzleSlot), With<ToFill>>,
    mut complete_puzzle: ResMut<Events<CompletePuzzle>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform, slot) in query.iter() {
        commands.despawn(entity);
        let puzzle = puzzles
            .towers
            .iter_mut()
            .find(|puzzle| puzzle.id == slot.puzzle_id)
            .unwrap();
        puzzle.filled += 1;
        if puzzle.filled == 4 {
            complete_puzzle.send(CompletePuzzle {
                coordinate: puzzle.coordinate.clone(),
                puzzle_id: puzzle.id,
            });
            continue;
        }

        let bundle: ShapeBundle = match slot.piece.form {
            EnemyForm::Circle => GeometryBuilder::build_as(
                &build_circle_path(),
                materials.add(slot.piece.color.to_color().into()),
                TessellationMode::Fill(FillOptions::default()),
                *transform,
            ),
            EnemyForm::Triangle => GeometryBuilder::build_as(
                &build_triangle_path(),
                materials.add(slot.piece.color.to_color().into()),
                TessellationMode::Fill(FillOptions::default()),
                *transform,
            ),
            EnemyForm::Quadratic => {
                let rectangle = shapes::Rectangle {
                    width: 18.0,
                    height: 18.0,
                    ..shapes::Rectangle::default()
                };
                let mut builder = GeometryBuilder::new();
                builder.add(&rectangle);
                builder.build(
                    materials.add(slot.piece.color.to_color().into()),
                    TessellationMode::Fill(FillOptions::default()),
                    *transform,
                )
            }
        };
        commands.spawn(bundle).with(PuzzleSlot {
            filled: true,
            ..slot.clone()
        });
    }
}

fn pick_up_piece(
    commands: &mut Commands,
    cursor: Res<Events<CursorMoved>>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut tamable_query: Query<(Entity, &mut Transform, &Enemy), With<Tameable>>,
    mut puzzle_query: Query<(Entity, &Transform, &mut PuzzleSlot)>,
    mut currently_picked: ResMut<CurrentPiece>,
    mut pick_source: ResMut<PickSource>,
) {
    let cursor_position = pick_source.cursor_events.latest(&cursor);
    let cursor_position = if let Some(cursor_position) = cursor_position {
        cursor_position.position - pick_source.cursor_offset
    } else {
        pick_source.last_cursor_pos
    };
    pick_source.last_cursor_pos = cursor_position;
    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        if currently_picked.entity.is_none() {
            for (entity, transform, enemy) in tamable_query.iter_mut() {
                if Vec2::new(
                    transform.translation.x - cursor_position.x,
                    transform.translation.y - cursor_position.y,
                )
                .length()
                    < 12.
                {
                    currently_picked.entity = Some(entity);
                    currently_picked.piece = Some(Piece {
                        form: enemy.form.clone(),
                        color: enemy.color.clone(),
                    });
                    return;
                }
            }
        } else {
            // we have a piece, place it in a puzzle or let it go
            let mut found_slot: bool = false;
            for (entity, transform, mut slot) in puzzle_query.iter_mut() {
                if slot.filled
                    || Vec2::new(
                        transform.translation.x - cursor_position.x,
                        transform.translation.y - cursor_position.y,
                    )
                    .length()
                        > 12.
                {
                    continue;
                }
                found_slot = true;
                if &slot.piece == currently_picked.piece.as_ref().unwrap() {
                    let (_, mut tamable_transform, _) = tamable_query
                        .get_mut(currently_picked.entity.unwrap())
                        .unwrap();
                    tamable_transform.translation = transform.translation;
                    commands.insert_one(entity, ToFill);
                    commands.despawn(currently_picked.entity.unwrap());
                    slot.filled = true;
                    currently_picked.entity = None;
                    currently_picked.piece = None;
                    return;
                }
            }
            if !found_slot {
                // go free my friend
                currently_picked.entity = None;
                currently_picked.piece = None;
            }
        }
    }
}

#[allow(dead_code)]
fn show_cursor(
    commands: &mut Commands,
    pick_source: Res<PickSource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut builder = PathBuilder::new();
    builder.arc(Vec2::new(0.0, 0.0), Vec2::new(3.0, 3.0), 2. * PI, 0.0);
    let path = builder.build();
    commands.spawn(GeometryBuilder::build_as(
        &path,
        materials.add(Color::BLACK.into()),
        TessellationMode::Fill(FillOptions::default()),
        Transform::from_translation(Vec3::new(
            pick_source.last_cursor_pos.x,
            pick_source.last_cursor_pos.y,
            10.,
        )),
    ));
}

fn update_picked_up_piece(
    pick_source: Res<PickSource>,
    currently_picked_up: Res<CurrentPiece>,
    mut enemy_query: Query<&mut Transform, With<Tameable>>,
) {
    if currently_picked_up.entity.is_none() {
        return;
    }
    if let Ok(mut transform) = enemy_query.get_mut(currently_picked_up.entity.unwrap()) {
        transform.translation = Vec3::new(
            pick_source.last_cursor_pos.x,
            pick_source.last_cursor_pos.y,
            0.,
        );
    }
}

fn update_puzzle(
    commands: &mut Commands,
    mut puzzles: ResMut<Puzzles>,
    mut my_event_reader: Local<EventReader<CompletePuzzle>>,
    my_events: Res<Events<CompletePuzzle>>,
    slot_query: Query<(Entity, &PuzzleSlot)>,
    mut puzzle_ids: ResMut<PuzzleIdFactory>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for completed_puzzle in my_event_reader.iter(&my_events) {
        let puzzle_id = completed_puzzle.puzzle_id;
        puzzles.towers = puzzles
            .towers
            .drain(..)
            .filter(|puzzle| puzzle.id != puzzle_id)
            .collect();
        for (entity, slot) in slot_query.iter() {
            if slot.puzzle_id == puzzle_id {
                commands.despawn(entity);
            }
        }
        let id = puzzle_ids.get_next_id();
        let puzzle = spawn_puzzle(
            id,
            completed_puzzle.coordinate.clone(),
            commands,
            &mut materials,
        );
        puzzles.towers.push(puzzle);
    }
}

fn break_down_puzzles(commands: &mut Commands, puzzle_slot_query: Query<Entity, With<PuzzleSlot>>) {
    for entity in puzzle_slot_query.iter() {
        commands.despawn(entity);
    }
}
