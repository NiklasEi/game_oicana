use crate::loading::FontAssets;
use crate::ui::ButtonColors;
use crate::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(click_play_button));
    }
}

#[derive(Component)]
struct PlayButton;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        })
        .insert(PlayButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
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
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<(Entity, &Interaction, &mut UiColor, &Children), With<Button>>,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut color, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                commands.entity(button).despawn();
                commands.entity(text).despawn();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}
