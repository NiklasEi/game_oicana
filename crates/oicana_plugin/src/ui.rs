use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GameState {
            health: 20,
            score: 0,
            enemy_health: 10,
        })
        .add_startup_system(init_life.system())
        .add_system(update_game_state.system())
        .add_system(show_lost.system());
    }
}

struct HealthText;

struct ScoreText;

pub struct GameState {
    pub health: usize,
    pub score: usize,
    pub enemy_health: i32,
}

fn init_life(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    game_state: Res<GameState>,
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
                        value: format!("Health: {}", game_state.health),
                        font,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.6, 0.6, 0.6),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(HealthText);
        });
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());
    commands
        .spawn(CameraUiBundle::default())
        // root node
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(10.),
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
                        value: format!("Score: {}", game_state.score),
                        font,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.6, 0.6, 0.6),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ScoreText);
        });
}

fn update_game_state(
    game_state: ChangedRes<GameState>,
    mut health_query: Query<(&mut Text, &HealthText)>,
    mut score_query: Query<(&mut Text, &ScoreText)>,
) {
    for (mut text, _tag) in health_query.iter_mut() {
        text.value = format!("Health: {}", game_state.health);
    }
    for (mut text, _tag) in score_query.iter_mut() {
        text.value = format!("Score: {}", game_state.score);
    }
}

fn show_lost(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    health: ChangedRes<GameState>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    if health.health < 1 {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let material = color_materials.add(Color::WHITE.into());
        commands
            .spawn(CameraUiBundle::default())
            // root node
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(170.),
                        top: Val::Px(200.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material,
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        value: "You lost! Restart to try again ;)".to_owned(),
                        font,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.6, 0.6, 0.6),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                });
            });
    }
}
