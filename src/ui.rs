use crate::AppState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .init_resource::<ButtonColors>()
            .add_systems(OnEnter(AppState::InGame), init_life)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_game_state)
                    .with_system(retry_system)
                    .with_system(click_retry_button),
            );
    }
}

#[derive(Resource)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct RetryButton;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
pub struct GameState {
    pub health: usize,
    pub score: usize,
    pub enemy_health: i32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            health: 20,
            score: 0,
            enemy_health: 1,
        }
    }
}

fn init_life(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    game_state: Res<GameState>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        format!("Health: {}", game_state.health),
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.6, 0.6, 0.6),
                            font,
                        },
                    ),
                    ..Default::default()
                })
                .insert(HealthText);
        });
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("Score: {}", game_state.score),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.6, 0.6, 0.6),
                                font,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}

fn update_game_state(
    game_state: Res<GameState>,
    mut health_query: Query<&mut Text, (With<HealthText>, Without<ScoreText>)>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<HealthText>)>,
) {
    if game_state.is_changed() {
        for mut text in health_query.iter_mut() {
            text.sections.first_mut().unwrap().value = format!("Health: {}", game_state.health);
        }
        for mut text in score_query.iter_mut() {
            text.sections.first_mut().unwrap().value = format!("Score: {}", game_state.score);
        }
    }
}

fn retry_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    game_state: Res<GameState>,
    button_materials: Res<ButtonColors>,
) {
    if game_state.is_changed() && game_state.health < 1 {
        commands
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: button_materials.normal.into(),
                ..Default::default()
            })
            .insert(RetryButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Restart".to_string(),
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        },
                    ),
                    ..Default::default()
                });
            });
    }
}

fn click_retry_button(
    mut commands: Commands,
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &Children),
        With<Button>,
    >,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut color, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *game_state = GameState::default();
                commands.entity(button).despawn();
                commands.entity(text).despawn();
                state.set(AppState::Restart);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
