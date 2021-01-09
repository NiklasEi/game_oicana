use crate::enemies::Enemy;
use crate::map::{Map, Tile};
use bevy::prelude::*;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_map_tower.system())
            .add_system(shoot.system());
    }
}

struct Tower {
    range: f32,
    damage: i32,
}

fn spawn_map_tower(commands: &mut Commands, map: Res<Map>) {
    let position_row = map
        .tiles
        .iter()
        .position(|row| row.contains(&Tile::Tower))
        .unwrap();
    let position_column = map
        .tiles
        .get(position_row)
        .unwrap()
        .iter()
        .position(|tile| tile == &Tile::Tower)
        .unwrap();
    commands.spawn((
        Tower {
            range: 200.,
            damage: 1,
        },
        Transform::from_translation(Vec3::new(
            position_column as f32 * map.tile_size,
            position_row as f32 * map.tile_size,
            0.,
        )),
    ));
}

fn shoot(
    tower_query: Query<(&Transform, &Tower)>,
    mut enemies_query: Query<(&Transform, &mut Enemy)>,
) {
    for (tower_pos, tower) in tower_query.iter() {
        'enemies: for (enemy_pos, mut enemy) in enemies_query.iter_mut() {
            let distance = enemy_pos.translation - tower_pos.translation;
            if distance.length() < tower.range {
                enemy.health -= tower.damage;
                break 'enemies;
            }
        }
    }
}
