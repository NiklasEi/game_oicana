use crate::enemies::{EnemyColor, EnemyForm};
use crate::map::{Coordinate, Map};
use bevy::prelude::*;

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_tower_puzzles.system());
    }
}

pub struct Puzzles {
    new_towers: Vec<Puzzle>,
}

pub struct Puzzle {
    coordinates: Coordinate,
    pieces: Vec<Piece>,
}

pub struct Piece {
    color: EnemyColor,
    form: EnemyForm,
}

fn update_tower_puzzles(towers: ResMut<Puzzles>, map: Res<Map>) {}
