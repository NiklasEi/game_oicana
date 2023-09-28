use crate::enemies::{Enemy, EnemyColor, EnemyForm, Tameable};
use crate::map::{Coordinate, Map, Tile};
use crate::{AppState, ENEMY_Z, PUZZLE_Z};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Circle;
use rand::random;

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PuzzleIdFactory::default())
            .insert_resource(PickSource {
                cursor_offset: Vec2::new(17., -19.),
                ..Default::default()
            })
            .insert_resource(CurrentPiece {
                entity: None,
                piece: None,
            })
            .add_event::<CompletePuzzle>()
            .insert_resource(Puzzles { towers: vec![] })
            .add_systems(OnEnter(AppState::InGame), set_tower_puzzles)
            .add_systems(
                Update,
                (
                    update_picked_up_piece,
                    (puzzle_input, place_puzzle_piece, update_puzzle).chain(),
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), break_down_puzzles);
    }
}

#[derive(Debug, Event)]
pub struct CompletePuzzle {
    pub coordinate: Coordinate,
    puzzle_id: usize,
}

#[derive(Default, Resource)]
struct PuzzleIdFactory {
    next_id: usize,
}

impl PuzzleIdFactory {
    pub fn get_next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id - 1
    }
}

#[derive(Clone, Component)]
pub struct PuzzleSlot {
    piece: Piece,
    filled: bool,
    puzzle_id: usize,
}

#[derive(Resource)]
pub struct CurrentPiece {
    pub entity: Option<Entity>,
    pub piece: Option<Piece>,
}

#[derive(Default, Resource)]
pub struct PickSource {
    pub last_cursor_pos: Vec2,
    pub cursor_offset: Vec2,
}

#[derive(Resource)]
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

#[derive(Component)]
struct ToFill;

fn set_tower_puzzles(
    mut commands: Commands,
    mut puzzles: ResMut<Puzzles>,
    map: Res<Map>,
    mut puzzle_ids: ResMut<PuzzleIdFactory>,
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
        let puzzle = spawn_puzzle(id, coordinate, &mut commands);

        puzzles.towers.push(puzzle);
    }
}

fn spawn_puzzle(id: usize, coordinate: Coordinate, commands: &mut Commands) -> Puzzle {
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

        let bundle = piece.form.build_bundle(
            Transform::from_translation(Vec3::new(coordinate.x, coordinate.y, PUZZLE_Z)),
            piece.color.to_color(),
            None,
        );
        commands.spawn(bundle).insert(PuzzleSlot {
            piece: piece.clone(),
            filled: false,
            puzzle_id: id,
        });
    }
    puzzle
}

fn place_puzzle_piece(
    mut commands: Commands,
    mut puzzles: ResMut<Puzzles>,
    mut query: Query<(Entity, &mut Fill, &mut PuzzleSlot), With<ToFill>>,
    mut complete_puzzle: EventWriter<CompletePuzzle>,
) {
    for (entity, mut fill, mut slot) in query.iter_mut() {
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

        commands.entity(entity).remove::<ToFill>();
        fill.color = slot.piece.color.to_color();
        slot.filled = true;
    }
}

fn puzzle_input(
    mut commands: Commands,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut tamable_query: Query<(Entity, &mut Transform, &Enemy), With<Tameable>>,
    mut puzzle_query: Query<(Entity, &Transform, &mut PuzzleSlot), Without<Enemy>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut currently_picked: ResMut<CurrentPiece>,
    mut pick_source: ResMut<PickSource>,
) {
    let (camera, camera_transform) = camera.single();
    let cursor_position = if let Some(world_position) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        world_position
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
            for (puzzle_entity, transform, mut slot) in puzzle_query.iter_mut() {
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
                    commands.entity(currently_picked.entity.unwrap()).despawn();
                    commands.entity(puzzle_entity).insert(ToFill);
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
fn show_cursor(mut commands: Commands, pick_source: Res<PickSource>) {
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&Circle {
                radius: 3.,
                center: Vec2::splat(0.),
            }),
            transform: Transform::from_translation(Vec3::new(
                pick_source.last_cursor_pos.x,
                pick_source.last_cursor_pos.y,
                10.,
            )),
            ..default()
        },
        Fill::color(Color::BLACK),
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
            ENEMY_Z,
        );
    }
}

fn update_puzzle(
    mut commands: Commands,
    mut puzzles: ResMut<Puzzles>,
    mut my_event_reader: EventReader<CompletePuzzle>,
    slot_query: Query<(Entity, &PuzzleSlot)>,
    mut puzzle_ids: ResMut<PuzzleIdFactory>,
) {
    for completed_puzzle in my_event_reader.iter() {
        let puzzle_id = completed_puzzle.puzzle_id;
        puzzles.towers = puzzles
            .towers
            .drain(..)
            .filter(|puzzle| puzzle.id != puzzle_id)
            .collect();
        for (entity, slot) in slot_query.iter() {
            if slot.puzzle_id == puzzle_id {
                commands.entity(entity).despawn();
            }
        }
        let id = puzzle_ids.get_next_id();
        let puzzle = spawn_puzzle(id, completed_puzzle.coordinate.clone(), &mut commands);
        puzzles.towers.push(puzzle);
    }
}

fn break_down_puzzles(mut commands: Commands, puzzle_slot_query: Query<Entity, With<PuzzleSlot>>) {
    for entity in puzzle_slot_query.iter() {
        commands.entity(entity).despawn();
    }
}
