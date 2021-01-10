use crate::enemies::{Enemy, Tameable};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_bullets.system());
    }
}

pub struct Bullet {
    pub damage: i32,
    pub speed: f32,
    pub target: Entity,
}

fn update_bullets(
    commands: &mut Commands,
    mut bullet_query: Query<(Entity, &Bullet, &mut Transform)>,
    mut enemy_query: Query<(&mut Enemy, &Transform), Without<Tameable>>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    for (entity, bullet, mut transform) in bullet_query.iter_mut() {
        let target = enemy_query.get_mut(bullet.target);
        if let Ok((mut target, target_transform)) = target {
            let distance = target_transform.translation - transform.translation;
            if distance.length() < bullet.speed * delta {
                target.health -= bullet.damage;
                commands.despawn(entity);
            } else {
                let movement = distance.normalize() * bullet.speed * delta;
                transform.translation += movement;
            }
        } else {
            commands.despawn(entity);
        }
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    bullet: Bullet,
    translation: Vec3,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let mut builder = PathBuilder::new();
    builder.arc(point(0.000001, 0.000001), 3., 3., 2. * PI, 0.1);
    let path = builder.build();
    commands
        .spawn(path.fill(
            materials.add(Color::BLACK.into()),
            meshes,
            translation,
            &FillOptions::default(),
        ))
        .with(bullet);
}
