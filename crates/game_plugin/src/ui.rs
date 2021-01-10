use bevy::app::AppExit;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Health { health: 100 })
            .add_startup_system(init_life.system())
            .add_system(show_life.system());
    }
}

struct HealthText;

pub struct Health {
    pub health: usize,
}

fn init_life(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    health: Res<Health>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());
    commands
        .spawn(CameraUiBundle::default())
        // root node
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: format!("Health: {}", health.health),
                        font,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(HealthText);
        });
}

fn show_life(
    health: ChangedRes<Health>,
    mut query: Query<(&mut Text, &HealthText)>,
    mut signal: ResMut<Events<AppExit>>,
) {
    for (mut text, _tag) in query.iter_mut() {
        text.value = format!("Health: {}", health.health);
    }
    if health.health < 1 {
        signal.send(AppExit);
    }
}
