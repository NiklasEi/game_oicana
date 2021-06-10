use crate::map::{Coordinate, Map};
use crate::puzzle::CurrentPiece;
use crate::ui::GameState;
use crate::AppState;
use bevy::prelude::*;
use bevy::utils::{HashMap, Instant};
use bevy_prototype_lyon::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;
use std::f32::consts::PI;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WaveState {
            last_spawn: Instant::now(),
        })
        .add_event::<EnemyBreach>()
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(remove_enemies.system())
                .with_system(spawn_enemies.system())
                .with_system(update_tamable_enemies.system())
                .with_system(update_enemies.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame).with_system(break_down_enemies.system()),
        );
    }
}

pub struct EnemyBreach;

struct WaveState {
    pub last_spawn: Instant,
}

pub struct Tameable;

pub struct Enemy {
    current_waypoint_index: usize,
    pub form: EnemyForm,
    pub color: EnemyColor,
    color_map: HashMap<i32, Color>,
    pub travelled: f32,
    pub max_health: i32,
}

pub struct Health {
    pub value: i32
}

pub struct Trees {
    pub coordinates: Vec<Coordinate>,
}

impl Enemy {
    pub fn get_color(&mut self, health: i32) -> Color {
        let cached_color = self.color_map.get(&health);
        if let Some(&color) = cached_color {
            return color.clone();
        }
        let health_factor = if health > 0 {
            health as f32 / self.max_health as f32
        } else {
            0.
        };
        let full_color = Color::GRAY * health_factor + self.color.to_color() * (1. - health_factor);

        self.color_map.insert(health, full_color.clone());
        full_color
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
    Lilac,
    Green,
    Blue,
    Pink,
}

impl Distribution<EnemyColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyColor {
        match rng.gen_range(0..4) {
            0 => EnemyColor::Red,
            1 => EnemyColor::Green,
            2 => EnemyColor::Blue,
            3 => EnemyColor::Pink,
            _ => EnemyColor::Lilac,
        }
    }
}

impl EnemyColor {
    pub fn to_color(&self) -> Color {
        match self {
            EnemyColor::Lilac => Color::rgb(84. / 255., 13. / 255., 110. / 255.),
            EnemyColor::Red => Color::rgb(235. / 255., 66. / 255., 102. / 255.),
            EnemyColor::Green => Color::rgb(83. / 255., 145. / 255., 126. / 255.),
            EnemyColor::Pink => Color::rgb(217. / 255., 154. / 255., 197. / 255.),
            EnemyColor::Blue => Color::rgb(88. / 255., 84. / 255., 129. / 255.),
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    map: Res<Map>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut wave_state: ResMut<WaveState>,
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
    let mut health = game_state.enemy_health;
    let one_percent = health / 100;
    let mut rng = rand::thread_rng();
    let percent: i32 = rng.gen_range(0..50); // generates a float between 0 and 1
    health += percent * one_percent;
    match form {
        EnemyForm::Circle => create_circle_enemy(&mut commands, color, &map, health),
        EnemyForm::Quadratic => create_quadratic_enemy(&mut commands, color, &map, health),
        EnemyForm::Triangle => create_triangle_enemy(&mut commands, color, &map, health),
    }
}

fn create_circle_enemy(commands: &mut Commands, color: EnemyColor, map: &Res<Map>, health: i32) {
    let geometry = build_circle_path();
    let mut enemy = Enemy {
        current_waypoint_index: 0,
        form: EnemyForm::Circle,
        max_health: health,
        color_map: HashMap::default(),
        color,
        travelled: 0.,
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &geometry,
            ShapeColors {
                main: enemy.get_color(health),
                outline: Color::DARK_GRAY,
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(map.spawn.x, map.spawn.y, 0.)),
        ))
        .insert(enemy).insert(Health {value: health});
}

pub fn build_circle_path() -> impl Geometry {
    let mut builder = PathBuilder::new();
    builder.arc(Vec2::new(0.001, 0.001), Vec2::new(10.0, 10.0), 2. * PI, 0.0);
    builder.build()
}

fn create_triangle_enemy(commands: &mut Commands, color: EnemyColor, map: &Res<Map>, health: i32) {
    let geometry = build_triangle_path();
    let mut enemy = Enemy {
        max_health: health,
        current_waypoint_index: 0,
        form: EnemyForm::Triangle,
        color_map: HashMap::default(),
        color,
        travelled: 0.,
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &geometry,
            ShapeColors {
                main: enemy.get_color(health),
                outline: Color::DARK_GRAY,
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(map.spawn.x, map.spawn.y, 0.)),
        ))
        .insert(enemy).insert(Health {value: health});
}

pub fn build_triangle_path() -> impl Geometry {
    let mut builder = PathBuilder::new();
    builder.move_to(Vec2::new(-5., 9.));
    builder.line_to(Vec2::new(-5., -9.));
    builder.line_to(Vec2::new(10., 0.));
    builder.line_to(Vec2::new(-5., 9.));
    builder.build()
}

fn create_quadratic_enemy(commands: &mut Commands, color: EnemyColor, map: &Res<Map>, health: i32) {
    let rectangle = shapes::Rectangle {
        width: 18.0,
        height: 18.0,
        ..shapes::Rectangle::default()
    };
    let mut builder = GeometryBuilder::new();
    builder.add(&rectangle);

    let mut enemy = Enemy {
        max_health: health,
        current_waypoint_index: 0,
        form: EnemyForm::Quadratic,
        color,
        color_map: HashMap::default(),
        travelled: 0.,
    };
    commands
        .spawn_bundle(builder.build(
            ShapeColors {
                main: enemy.get_color(health),
                outline: Color::DARK_GRAY,
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(map.spawn.x, map.spawn.y, 0.)),
        ))
        .insert(enemy).insert(Health {value: health});
}

fn remove_enemies(
    mut commands: Commands,
    map: Res<Map>,
    mut game_state: ResMut<GameState>,
    mut enemy_breach: EventWriter<EnemyBreach>,
    enemy_query: Query<(Entity, &Enemy, &Health), Without<Tameable>>,
) {
    for (entity, enemy, health) in enemy_query.iter() {
        if health.value < 0 {
            if game_state.health > 0 {
                game_state.score += enemy.max_health as usize;
            }
            commands.entity(entity).insert(Tameable);
            continue;
        }
        if enemy.current_waypoint_index >= map.waypoints.len() {
            if game_state.health > 0 {
                game_state.health -= 1;
                enemy_breach.send(EnemyBreach);
            }
            commands.entity(entity).despawn();
            continue;
        }
    }
}

fn update_enemies(
    time: Res<Time>,
    map: Res<Map>,
    mut enemy_query: Query<
        (&mut Enemy, &mut Transform),
        Without<Tameable>,
    >,
) {
    let delta = time.delta().as_millis() as f32;
    let speed = 0.1;
    for (mut enemy, mut transform) in enemy_query.iter_mut() {
        if enemy.current_waypoint_index >= map.waypoints.len() {
            continue;
        }
        let destination = map.waypoints.get(enemy.current_waypoint_index).unwrap();
        let distance = Vec3::new(destination.x, destination.y, 0.) - transform.translation;
        if distance == Vec3::ZERO {
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

fn update_enemy_colors(mut commands: Commands,
                       mut materials: ResMut<Assets<ColorMaterial>>,) {

}

fn update_tamable_enemies(
    mut commands: Commands,
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
                commands.entity(entity).despawn();
            } else {
                transform.translation += movement;
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}

fn break_down_enemies(mut commands: Commands, enemies_query: Query<Entity, With<Enemy>>) {
    for entity in enemies_query.iter() {
        commands.entity(entity).despawn();
    }
}
