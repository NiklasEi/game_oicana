use crate::bullets::{spawn_bullet, Bullet};
use crate::enemies::{Enemy, Tameable};
use crate::map::{Coordinate, Map, MapTile, Tile};
use crate::puzzle::CompletePuzzle;
use crate::{AppState, STAGE};
use bevy::prelude::*;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<TowerShot>()
            .on_state_enter(STAGE, AppState::InGame, spawn_map_tower.system())
            .on_state_update(STAGE, AppState::InGame, shoot.system())
            .on_state_update(STAGE, AppState::InGame, build_and_upgrade_towers.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_towers.system());
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
                    level: 1,
                    speed: 200.,
                    coordinate: coordinate.clone(),
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
    mut tower_shot: ResMut<Events<TowerShot>>,
    mut enemies_query: Query<(Entity, &Transform, &mut Enemy), Without<Tameable>>,
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
                spawn_bullet(
                    commands,
                    bullet,
                    tower_pos.translation,
                    &mut materials,
                    &mut meshes,
                );
                tower_shot.send(TowerShot);
            }
        }
    }
}

fn build_and_upgrade_towers(
    commands: &mut Commands,
    mut event_reader: Local<EventReader<CompletePuzzle>>,
    completed_puzzle: Res<Events<CompletePuzzle>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tower_query: Query<(&mut Tower, &mut Timer)>,
    mut map_tiles_query: Query<(&Transform, &mut Handle<ColorMaterial>), With<MapTile>>,
) {
    for completed_puzzle in event_reader.iter(&completed_puzzle) {
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
            let tower_handle: Handle<Texture> = asset_server.load("tower64x64.png");
            for (transform, mut material) in map_tiles_query.iter_mut() {
                if transform.translation.x == coordinate.x
                    && transform.translation.y == coordinate.y
                {
                    *material = materials.add(tower_handle.clone().into())
                }
            }
            commands
                .spawn((
                    Tower {
                        range: 100.,
                        damage: 15,
                        level: 1,
                        speed: 200.,
                        coordinate: coordinate.clone(),
                    },
                    Transform::from_translation(Vec3::new(coordinate.x, coordinate.y, 0.)),
                ))
                .with(Timer::from_seconds(0.3, true));
        }
    }
}

fn break_down_towers(commands: &mut Commands, tower_query: Query<Entity, With<Tower>>) {
    for entity in tower_query.iter() {
        commands.despawn(entity);
    }
}
