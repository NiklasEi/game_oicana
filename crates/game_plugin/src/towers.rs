use crate::bullets::{spawn_bullet, Bullet};
use crate::enemies::Enemy;
use crate::map::{Coordinate, Map, Tile};
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
    let mut tower_positions: Vec<Coordinate> = vec![];

    for (row_index, row) in map.tiles.iter().enumerate() {
        for (column_index, tile) in row.iter().enumerate() {
            if tile == &Tile::Tower {
                tower_positions.push(Coordinate {
                    x: column_index as f32 * map.tile_size,
                    y: row_index as f32 * map.tile_size,
                })
            }
        }
    }

    for coordinate in tower_positions {
        commands
            .spawn((
                Tower {
                    range: 100.,
                    damage: 15,
                },
                Transform::from_translation(Vec3::new(coordinate.x, coordinate.y, 0.)),
            ))
            .with(Timer::from_seconds(0.3, true));
    }
}

fn shoot(
    commands: &mut Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tower_query: Query<(&Transform, &Tower, &mut Timer)>,
    mut enemies_query: Query<(Entity, &Transform, &mut Enemy)>,
) {
    for (tower_pos, tower, mut timer) in tower_query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.just_finished() {
            let furthest_target: Option<(Entity, f32)> = enemies_query
                .iter_mut()
                .filter(|(_, pos, _)| {
                    let distance = pos.translation - tower_pos.translation;
                    distance.length() < tower.range
                })
                .fold(None, |acc, (entity, pos, enemy)| {
                    if let Some((_, old_travelled)) = acc {
                        if enemy.travelled > old_travelled {
                            Some((entity.clone(), enemy.travelled))
                        } else {
                            acc
                        }
                    } else {
                        Some((entity.clone(), enemy.travelled))
                    }
                });

            if let Some((target, _)) = furthest_target {
                let bullet = Bullet {
                    damage: tower.damage,
                    speed: 200.,
                    target,
                };
                spawn_bullet(
                    commands,
                    bullet,
                    tower_pos.translation,
                    &mut materials,
                    &mut meshes,
                );
            }
        }
    }
}
