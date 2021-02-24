use crate::{AppState, STAGE};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GameState::default())
            .init_resource::<ButtonMaterials>()
            .on_state_enter(STAGE, AppState::InGame, init_life.system())
            .on_state_update(STAGE, AppState::InGame, update_game_state.system())
            .on_state_update(STAGE, AppState::InGame, retry_system.system())
            .on_state_update(STAGE, AppState::InGame, click_retry_button.system());
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

struct RetryButton;

struct HealthText;

struct ScoreText;

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
    mut health_query: Query<&mut Text, With<HealthText>>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in health_query.iter_mut() {
        text.value = format!("Health: {}", game_state.health);
    }
    for mut text in score_query.iter_mut() {
        text.value = format!("Score: {}", game_state.score);
    }
}

fn retry_system(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    game_state: ChangedRes<GameState>,
    button_materials: Res<ButtonMaterials>,
) {
    if game_state.health < 1 {
        commands
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with(RetryButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        value: "Restart".to_string(),
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                });
            });
    }
}

fn click_retry_button(
    commands: &mut Commands,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *game_state = GameState::default();
                commands.despawn(button);
                commands.despawn(text);
                state.set_next(AppState::Restart).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
