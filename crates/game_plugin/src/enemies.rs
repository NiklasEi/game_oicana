use crate::map::{Coordinate, Map, Tile};
use crate::puzzle::CurrentPiece;
use crate::ui::GameState;
use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::utils::{HashMap, Instant};
use bevy_prototype_lyon::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;
use std::f32::consts::{FRAC_PI_6, PI};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(WaveState {
            last_spawn: Instant::now(),
        })
        .add_system(remove_enemies.system())
        .add_system(spawn_enemies.system())
        .add_system(update_tamable_enemies.system())
        .add_system(update_enemies.system());
    }
}

struct WaveState {
    pub last_spawn: Instant,
}

pub struct Tameable;

pub struct Enemy {
    current_waypoint_index: usize,
    pub form: EnemyForm,
    pub color: EnemyColor,
    color_handle_map: HashMap<i32, HandleId>,
    pub travelled: f32,
    pub health: i32,
    pub max_health: i32,
}

pub struct Trees {
    pub coordinates: Vec<Coordinate>,
}

impl Enemy {
    pub fn get_color_handle(
        &mut self,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Handle<ColorMaterial> {
        let cached_color_handle_id = self.color_handle_map.get(&self.health);
        if let Some(&handle) = cached_color_handle_id {
            return materials.get_handle(handle);
        }
        let health_factor = self.health as f32 / self.max_health as f32;
        let full_color =
            Color::GRAY * health_factor * 2. + self.color.to_color() * (1. - health_factor);

        let color_handle = materials.add(full_color.into());
        self.color_handle_map.insert(self.health, color_handle.id);
        color_handle
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnemyForm {
    Circle,
    Triangle,
    Quadratic,
}

impl Distribution<EnemyForm> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyForm {
        match rng.gen_range(0..3) {
            0 => EnemyForm::Circle,
            1 => EnemyForm::Triangle,
            _ => EnemyForm::Quadratic,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnemyColor {
    Red,
    Blue,
}

impl Distribution<EnemyColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyColor {
        match rng.gen_range(0..2) {
            0 => EnemyColor::Red,
            _ => EnemyColor::Blue,
        }
    }
}

impl EnemyColor {
    pub fn to_color(&self) -> Color {
        match self {
            EnemyColor::Blue => Color::BLUE,
            EnemyColor::Red => Color::RED,
        }
    }
}

fn spawn_enemies(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    map: Res<Map>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut wave_state: ResMut<WaveState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if time.last_update().is_some()
        && time
            .last_update()
            .unwrap()
            .duration_since(wave_state.last_spawn)
            .as_secs_f32()
            < 1.
    {
        return;
    } else if time.last_update().is_some() {
        wave_state.last_spawn = time.last_update().unwrap();
    }
    if game_state.health < 1 {
        return;
    }
    game_state.enemy_health += 1;
    let form: EnemyForm = random();
    let color: EnemyColor = random();
    match form {
        EnemyForm::Circle => create_circle_enemy(
            commands,
            &mut materials,
            color,
            &map,
            &game_state,
            &mut meshes,
        ),
        EnemyForm::Quadratic => create_quadratic_enemy(
            commands,
            &mut materials,
            color,
            &map,
            &game_state,
            &mut meshes,
        ),
        EnemyForm::Triangle => create_triangle_enemy(
            commands,
            &mut materials,
            color,
            &map,
            &game_state,
            &mut meshes,
        ),
    }
}

fn create_circle_enemy(
    commands: &mut Commands,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    color: EnemyColor,
    map: &Res<Map>,
    game_state: &GameState,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let path = build_circle_path();
    let mut enemy = Enemy {
        current_waypoint_index: 0,
        form: EnemyForm::Circle,
        health: game_state.enemy_health,
        max_health: game_state.enemy_health,
        color_handle_map: HashMap::default(),
        color,
        travelled: 0.,
    };
    commands
        .spawn(path.fill(
            enemy.get_color_handle(materials),
            meshes,
            Vec3::new(map.spawn.x, map.spawn.y, 0.),
            &FillOptions::default(),
        ))
        .with(enemy);
}

pub fn build_circle_path() -> Path {
    let mut builder = PathBuilder::new();
    builder.arc(point(0.000001, 0.000001), 10., 10., 2. * PI, 0.1);
    builder.build()
}

fn create_triangle_enemy(
    commands: &mut Commands,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    color: EnemyColor,
    map: &Res<Map>,
    game_state: &GameState,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let path = build_triangle_path();
    let mut enemy = Enemy {
        health: game_state.enemy_health,
        max_health: game_state.enemy_health,
        current_waypoint_index: 0,
        form: EnemyForm::Triangle,
        color_handle_map: HashMap::default(),
        color,
        travelled: 0.,
    };
    commands
        .spawn(path.fill(
            enemy.get_color_handle(&mut materials),
            meshes,
            Vec3::new(map.spawn.x, map.spawn.y, 0.),
            &FillOptions::default(),
        ))
        .with(enemy);
}

pub fn build_triangle_path() -> Path {
    let mut builder = PathBuilder::new();
    builder.move_to(point(-5., 9.));
    builder.line_to(point(-5., -9.));
    builder.line_to(point(10., 0.));
    builder.line_to(point(-5., 9.));
    builder.build()
}

fn create_quadratic_enemy(
    commands: &mut Commands,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    color: EnemyColor,
    map: &Res<Map>,
    game_state: &GameState,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let path = build_quadratic_path();
    let mut enemy = Enemy {
        health: game_state.enemy_health,
        max_health: game_state.enemy_health,
        current_waypoint_index: 0,
        form: EnemyForm::Quadratic,
        color,
        color_handle_map: HashMap::default(),
        travelled: 0.,
    };
    commands
        .spawn(path.fill(
            enemy.get_color_handle(&mut materials),
            meshes,
            Vec3::new(map.spawn.x, map.spawn.y, 0.),
            &FillOptions::default(),
        ))
        .with(enemy);
}

pub fn build_quadratic_path() -> Path {
    let mut builder = PathBuilder::new();
    builder.move_to(point(-9., 9.));
    builder.line_to(point(-9., -9.));
    builder.line_to(point(9., -9.));
    builder.line_to(point(9., 9.));
    builder.line_to(point(-9., 9.));
    builder.build()
}

fn remove_enemies(
    commands: &mut Commands,
    map: Res<Map>,
    mut game_state: ResMut<GameState>,
    mut enemy_query: Query<(Entity, &Enemy), Without<Tameable>>,
) {
    for (entity, enemy) in enemy_query.iter() {
        if enemy.health < 0 {
            if game_state.health > 0 {
                game_state.score += enemy.max_health as usize;
            }
            commands.insert_one(entity, Tameable);
            continue;
        }
        if enemy.current_waypoint_index >= map.waypoints.len() {
            if game_state.health > 0 {
                game_state.health -= 1;
            }
            commands.despawn(entity);
            continue;
        }
    }
}

fn update_enemies(
    time: Res<Time>,
    map: Res<Map>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut enemy_query: Query<
        (&mut Enemy, &mut Transform, &mut Handle<ColorMaterial>),
        Without<Tameable>,
    >,
) {
    let delta = time.delta().as_millis() as f32;
    let speed = 0.1;
    for (mut enemy, mut transform, mut color) in enemy_query.iter_mut() {
        if enemy.current_waypoint_index >= map.waypoints.len() {
            continue;
        }
        *color = enemy.get_color_handle(&mut materials);
        let destination = map.waypoints.get(enemy.current_waypoint_index).unwrap();
        let distance = Vec3::new(destination.x, destination.y, 0.) - transform.translation;
        if distance == Vec3::zero() {
            enemy.current_waypoint_index += 1;
            continue;
        }
        let movement = distance.normalize() * delta * speed;
        if movement.length() > distance.length() {
            transform.translation = Vec3::new(destination.x, destination.y, 0.);
            enemy.travelled += distance.length();
            enemy.current_waypoint_index += 1;
        } else {
            enemy.travelled += movement.length();
            transform.translation += movement;
        }
    }
}

fn update_tamable_enemies(
    commands: &mut Commands,
    time: Res<Time>,
    trees: Res<Trees>,
    currently_picked_up: Res<CurrentPiece>,
    mut enemy_query: Query<(Entity, &mut Transform), With<Tameable>>,
) {
    let delta = time.delta().as_secs_f32();
    let speed = 50.;
    for (entity, mut transform) in enemy_query.iter_mut() {
        if let Some(picked_entity) = currently_picked_up.entity {
            if picked_entity == entity {
                continue;
            }
        }
        let (_, closest_tree_position) = trees.coordinates.iter().fold(
            (10_000., Coordinate { x: 0., y: 0. }),
            |acc, coordinate| {
                let (old_distance, _) = acc;
                let distance =
                    (Vec3::new(coordinate.x, coordinate.y, 0.) - transform.translation).length();
                if distance < old_distance {
                    (distance, coordinate.clone())
                } else {
                    acc
                }
            },
        );
        let direction =
            Vec3::new(closest_tree_position.x, closest_tree_position.y, 0.) - transform.translation;
        if direction.is_finite() {
            let movement = direction.normalize() * delta * speed;
            if movement.length() > direction.length() {
                commands.despawn(entity);
            } else {
                transform.translation += movement;
            }
        } else {
            commands.despawn(entity);
        }
    }
}
