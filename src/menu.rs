use crate::loading::FontAssets;
use crate::ui::ButtonColors;
use crate::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(AppState::Menu)));
    }
}

#[derive(Component)]
struct PlayButton;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .insert(PlayButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn click_play_button(
    mut commands: Commands,
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<AppState>>,
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
                commands.entity(button).despawn();
                commands.entity(text).despawn();
                state.set(AppState::InGame);
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
