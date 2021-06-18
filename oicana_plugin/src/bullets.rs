use crate::enemies::{EnemyLabels, Health, Tameable};
use crate::AppState;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(update_bullets.system().label(EnemyLabels::Damage)),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame).with_system(break_down_bullets.system()),
        );
    }
}

pub struct Bullet {
    pub damage: i32,
    pub speed: f32,
    pub target: Entity,
}

fn update_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Bullet, &mut Transform)>,
    mut enemy_query: Query<(&Transform, &mut Health), (Without<Tameable>, Without<Bullet>)>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    for (entity, bullet, mut transform) in bullet_query.iter_mut() {
        let target = enemy_query.get_mut(bullet.target);
        if let Ok((target_transform, mut health)) = target {
            let distance = target_transform.translation - transform.translation;
            if distance.length() < bullet.speed * delta {
                health.value -= bullet.damage;
                commands.entity(entity).despawn();
            } else {
                let movement = distance.normalize() * bullet.speed * delta;
                transform.translation += movement;
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_bullet(commands: &mut Commands, bullet: Bullet, translation: Vec3) {
    let mut builder = PathBuilder::new();
    builder.arc(Vec2::new(0.001, 0.001), Vec2::new(3.0, 3.0), 2. * PI, 0.0);
    let path = builder.build();
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &path,
            ShapeColors {
                main: Color::BLACK,
                outline: Color::BLACK,
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(translation),
        ))
        .insert(bullet);
}

fn break_down_bullets(mut commands: Commands, bullets_query: Query<Entity, With<Bullet>>) {
    for entity in bullets_query.iter() {
        commands.entity(entity).despawn();
    }
}
