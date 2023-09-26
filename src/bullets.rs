use crate::enemies::{Enemy, EnemyLabels, Health, Tameable};
use crate::AppState;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Circle;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_bullets
                .label(EnemyLabels::Damage)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), break_down_bullets);
    }
}

#[derive(Component)]
pub struct Bullet {
    pub damage: i32,
    pub speed: f32,
}

fn update_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Bullet, &mut Transform), Without<Enemy>>,
    mut enemy_query: Query<(&Transform, &mut Health, &mut Enemy), Without<Tameable>>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    for (target_transform, mut health, mut enemy) in enemy_query.iter_mut() {
        let mut to_remove: Vec<Entity> = vec![];
        for bullet_id in enemy.bullets.iter() {
            let bullet = bullet_query.get_mut(*bullet_id);
            if let Ok((bullet_entity, bullet, mut transform)) = bullet {
                let mut distance = target_transform.translation - transform.translation;
                distance.z = 0.;
                if distance.length() < bullet.speed * delta {
                    health.value -= bullet.damage;
                    commands.entity(bullet_entity).despawn();
                    to_remove.push(*bullet_id);
                } else {
                    let movement = distance.normalize() * bullet.speed * delta;
                    transform.translation += movement;
                }
            } else {
                to_remove.push(*bullet_id);
            }
        }
        enemy.bullets = enemy
            .bullets
            .drain(..)
            .filter(|entity| !to_remove.contains(entity))
            .collect();
    }
}

pub fn spawn_bullet(commands: &mut Commands, bullet: Bullet, translation: Vec3) -> Entity {
    commands
        .spawn(GeometryBuilder::build_as(
            &Circle {
                radius: 3.,
                center: Vec2::splat(0.),
            },
            DrawMode::Fill(FillMode::color(Color::BLACK)),
            Transform::from_translation(translation),
        ))
        .insert(bullet)
        .id()
}

fn break_down_bullets(mut commands: Commands, bullets_query: Query<Entity, With<Bullet>>) {
    for entity in bullets_query.iter() {
        commands.entity(entity).despawn();
    }
}
