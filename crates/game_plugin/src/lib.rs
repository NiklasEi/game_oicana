use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "TD with puzzles".to_string(),
            ..Default::default()
        })
            .add_resource(ClearColor(Color::ALICE_BLUE))
            .add_startup_system(setup_camera_and_light.system());
    }
}

fn setup_camera_and_light(commands: &mut Commands) {
    commands
        // Camera
        .spawn(Camera2dBundle {
            ..Default::default()
        })
        // Light
        .spawn(LightBundle {
            ..Default::default()
        });
}
