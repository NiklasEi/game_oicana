use crate::loading::FontAssets;
use crate::{AppState, STAGE};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .on_state_enter(STAGE, AppState::Menu, setup_menu.system())
            .on_state_update(STAGE, AppState::Menu, click_play_button.system());
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

struct PlayButton;

fn setup_menu(
    commands: &mut Commands,
    font_assets: Res<FontAssets>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with(PlayButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Play".to_string(),
                    font: font_assets.fira_sans.clone(),
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

fn click_play_button(
    commands: &mut Commands,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<AppState>>,
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
                commands.despawn(button);
                commands.despawn(text);
                state.set_next(AppState::InGame).unwrap();
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
