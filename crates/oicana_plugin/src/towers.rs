use crate::bullets::{spawn_bullet, Bullet};
use crate::enemies::{Enemy, Tameable};
use crate::loading::TextureAssets;
use crate::map::{Coordinate, Map, MapTile, Tile};
use crate::puzzle::CompletePuzzle;
use crate::AppState;
use bevy::prelude::*;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<TowerShot>()
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(spawn_map_tower.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(shoot.system())
                    .with_system(build_and_upgrade_towers.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(break_down_towers.system()),
            );
    }
}

pub struct TowerShot;

struct Tower {
    level: usize,
    range: f32,
    damage: i32,
    speed: f32,
    coordinate: Coordinate,
}

fn spawn_map_tower(mut commands: Commands, map: Res<Map>) {
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
            .spawn_bundle((
                Tower {
                    range: 100.,
                    damage: 15,
                    level: 1,
                    speed: 200.,
                    coordinate: coordinate.clone(),
                },
                Transform::from_translation(Vec3::new(coordinate.x, coordinate.y, 0.)),
            ))
            .insert(Timer::from_seconds(0.3, true));
    }
}

fn shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut tower_query: Query<(&Transform, &Tower, &mut Timer)>,
    mut tower_shot: EventWriter<TowerShot>,
    mut enemies_query: Query<(Entity, &Transform, &mut Enemy), Without<Tameable>>,
) {
    for (tower_pos, tower, mut timer) in tower_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let furthest_target: Option<(Entity, f32)> = enemies_query
                .iter_mut()
                .filter(|(_, pos, _)| {
                    let distance = pos.translation - tower_pos.translation;
                    distance.length() < tower.range
                })
                .fold(None, |acc, (entity, _, enemy)| {
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
                    speed: tower.speed,
                    target,
                };
                spawn_bullet(&mut commands, bullet, tower_pos.translation);
                tower_shot.send(TowerShot);
            }
        }
    }
}

fn build_and_upgrade_towers(
    mut commands: Commands,
    mut event_reader: EventReader<CompletePuzzle>,
    texture_assets: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tower_query: Query<(&mut Tower, &mut Timer)>,
    mut map_tiles_query: Query<(&Transform, &mut Handle<ColorMaterial>), With<MapTile>>,
) {
    for completed_puzzle in event_reader.iter() {
        let coordinate: Coordinate = completed_puzzle.coordinate.clone();
        if let Some((mut tower, mut timer)) = tower_query
            .iter_mut()
            .find(|(tower, _timer)| tower.coordinate == coordinate)
        {
            tower.level += 1;
            tower.speed += 20.;
            tower.damage += 5;
            tower.range += 5.;

            *timer = Timer::from_seconds(if tower.level == 2 { 0.2 } else { 0.1 }, true);
        } else {
            for (transform, mut material) in map_tiles_query.iter_mut() {
                if transform.translation.x == coordinate.x
                    && transform.translation.y == coordinate.y
                {
                    *material = materials.add(texture_assets.tower.clone().into())
                }
            }
            commands
                .spawn_bundle((
                    Tower {
                        range: 100.,
                        damage: 15,
                        level: 1,
                        speed: 200.,
                        coordinate: coordinate.clone(),
                    },
                    Transform::from_translation(Vec3::new(coordinate.x, coordinate.y, 0.)),
                ))
                .insert(Timer::from_seconds(0.3, true));
        }
    }
}

fn break_down_towers(mut commands: Commands, tower_query: Query<Entity, With<Tower>>) {
    for entity in tower_query.iter() {
        commands.entity(entity).despawn();
    }
}
