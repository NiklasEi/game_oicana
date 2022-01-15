use crate::bullets::{spawn_bullet, Bullet};
use crate::enemies::{Enemy, Tameable};
use crate::loading::TextureAssets;
use crate::map::{Coordinate, Map, MapTile, Tile};
use crate::puzzle::CompletePuzzle;
use crate::{AppState, TOWER_Z};
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TowerShot>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_map_tower))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(shoot)
                    .with_system(build_and_upgrade_towers),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(break_down_towers));
    }
}

pub struct TowerShot;

#[derive(Component)]
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
        commands.spawn_bundle(TowerBundle::new(coordinate));
    }
}

fn shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut tower_query: Query<(&Transform, &Tower, &mut TowerCooldown)>,
    mut tower_shot: EventWriter<TowerShot>,
    mut enemies_query: Query<(Entity, &Transform, &mut Enemy), Without<Tameable>>,
) {
    for (tower_pos, tower, mut tower_cooldown) in tower_query.iter_mut() {
        tower_cooldown.tick(time.delta());
        if tower_cooldown.just_finished() {
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
                let (_, _, mut enemy) = enemies_query.get_mut(target).unwrap();
                let bullet = Bullet {
                    damage: tower.damage,
                    speed: tower.speed,
                };
                enemy.bullets.push({
                    let mut translation = tower_pos.translation;
                    translation.z += 2.;
                    spawn_bullet(&mut commands, bullet, translation)
                });
                tower_shot.send(TowerShot);
            }
        }
    }
}

fn build_and_upgrade_towers(
    mut commands: Commands,
    mut event_reader: EventReader<CompletePuzzle>,
    texture_assets: Res<TextureAssets>,
    mut tower_query: Query<(&mut Tower, &mut TowerCooldown)>,
    mut map_tiles_query: Query<(&Transform, &mut Handle<Image>), With<MapTile>>,
) {
    for completed_puzzle in event_reader.iter() {
        let coordinate: Coordinate = completed_puzzle.coordinate.clone();
        if let Some((mut tower, mut tower_cooldown)) = tower_query
            .iter_mut()
            .find(|(tower, _)| tower.coordinate == coordinate)
        {
            tower.level += 1;
            tower.speed += 20.;
            tower.damage += 5;
            tower.range += 5.;

            *tower_cooldown = TowerCooldown(Timer::from_seconds(
                if tower.level == 2 { 0.2 } else { 0.1 },
                true,
            ));
        } else {
            for (transform, mut image) in map_tiles_query.iter_mut() {
                if transform.translation.x == coordinate.x
                    && transform.translation.y == coordinate.y
                {
                    *image = texture_assets.tower.clone().into()
                }
            }
            commands.spawn_bundle(TowerBundle::new(coordinate));
        }
    }
}

#[derive(Component)]
pub struct TowerCooldown(Timer);

impl Deref for TowerCooldown {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TowerCooldown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for TowerCooldown {
    fn default() -> Self {
        TowerCooldown(Timer::from_seconds(0.3, true))
    }
}

#[derive(Bundle)]
pub struct TowerBundle {
    tower: Tower,
    transform: Transform,
    cooldown: TowerCooldown,
}

impl TowerBundle {
    fn new(coordinate: Coordinate) -> Self {
        TowerBundle {
            tower: Tower {
                range: 100.,
                damage: 15,
                level: 1,
                speed: 200.,
                coordinate: coordinate.clone(),
            },
            transform: Transform::from_translation(coordinate.to_translation(TOWER_Z)),
            cooldown: TowerCooldown::default(),
        }
    }
}

fn break_down_towers(
    mut commands: Commands,
    tower_query: Query<(Entity, &Tower)>,
    mut map_tiles_query: Query<(&Transform, &mut Handle<ColorMaterial>, &MapTile)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    texture_assets: Res<TextureAssets>,
) {
    for (entity, tower) in tower_query.iter() {
        for (transform, mut material, map_tile) in map_tiles_query.iter_mut() {
            if transform.translation.x == tower.coordinate.x
                && transform.translation.y == tower.coordinate.y
                && map_tile.tile != Tile::Tower
            {
                *material = materials.add(texture_assets.tower_plot.clone().into())
            }
        }
        commands.entity(entity).despawn();
    }
}
