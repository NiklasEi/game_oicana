use crate::map::Map;
use bevy::prelude::*;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_enemies.system())
            .add_system(update_enemies.system());
    }
}

struct Enemy {
    current_waypoint_index: usize,
}

fn spawn_enemies(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemy_handle: Handle<Texture> = asset_server.load("towerplot64x64.png");
    let mut transform = Transform::from_scale(Vec3::new(0.2, 0.2, 0.2));
    transform.translation = Vec3::new(map.spawn.x, map.spawn.y, 0.);
    commands
        .spawn(SpriteBundle {
            material: materials.add(enemy_handle.clone().into()),
            transform,
            ..Default::default()
        })
        .with(Enemy {
            current_waypoint_index: 0,
        });
}

fn update_enemies(
    time: Res<Time>,
    map: Res<Map>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform)>,
) {
    let delta = time.delta().as_millis() as f32;
    let speed = 0.1;
    for (mut enemy, mut transform) in enemy_query.iter_mut() {
        if enemy.current_waypoint_index >= map.waypoints.len() {
            continue;
        }
        let destination = map.waypoints.get(enemy.current_waypoint_index).unwrap();
        let distance = Vec3::new(destination.x, destination.y, 0.) - transform.translation;
        if distance == Vec3::zero() {
            enemy.current_waypoint_index += 1;
            continue;
        }
        let movement = distance.normalize() * delta * speed;
        println!(
            "dest: {:?}, distance {:?}, movement {:?}",
            destination, distance, movement
        );
        if movement.length() > distance.length() {
            transform.translation = Vec3::new(destination.x, destination.y, 0.);
            enemy.current_waypoint_index += 1;
        } else {
            transform.translation += movement;
        }
    }
}
