use crate::enemies::EnemyBreach;
use crate::towers::TowerShot;
use crate::{AppState, STAGE};
use bevy::prelude::{
    AppBuilder, EventReader, Events, IntoSystem, Local, Plugin, Res, ResMut
};
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::handle::SoundHandle;
use kira::sound::SoundSettings;
use kira::arrangement::Arrangement;
use kira::sequence::Sequence;
use kira::arrangement::handle::ArrangementHandle;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
        let background = audio_manager.load_sound("assets/sounds/background.ogg", SoundSettings::default()).unwrap();
        let shot = audio_manager.load_sound("assets/sounds/shot.ogg", SoundSettings::default()).unwrap();
        let breach = audio_manager.load_sound("assets/sounds/enemybreach.ogg", SoundSettings::default()).unwrap();
        app
            .add_resource(Audio {
                manager: audio_manager,
                background_handle: None
            })
            .add_resource(Sounds {
        background,
        shot,
        breach})
            .on_state_enter(STAGE, AppState::InGame, start_audio.system())
            .on_state_update(STAGE, AppState::InGame, tower_shots.system())
            .on_state_update(STAGE, AppState::InGame, enemy_breach.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_audio.system());
    }
}

struct Audio {
    manager: AudioManager,
    background_handle: Option<ArrangementHandle>
}

struct Sounds {
    background: SoundHandle,
    shot: SoundHandle,
    breach: SoundHandle,
}

fn start_audio(sound: Res<Sounds>, mut audio: ResMut<Audio>) {
    let loop_id = audio.manager
        .add_arrangement(Arrangement::new_loop(&sound.background, Default::default())).unwrap();
    let mut sequence = Sequence::<()>::new(Default::default());
    sequence.play(&loop_id, Default::default());
    audio.background_handle = Some(loop_id);
    audio.manager.start_sequence(sequence, Default::default()).unwrap();
}

fn tower_shots(
    mut tower_shot_reader: Local<EventReader<TowerShot>>,
    tower_shot: Res<Events<TowerShot>>,
    sound: Res<Sounds>, mut audio: ResMut<Audio>
) {
    if tower_shot_reader.latest(&tower_shot).is_some() {
        let mut sequence = Sequence::<()>::new(Default::default());
        sequence.play(&sound.shot, Default::default());
        audio.manager.start_sequence(sequence, Default::default()).unwrap();
    }
}

fn enemy_breach(
    mut enemy_breach_reader: Local<EventReader<EnemyBreach>>,
    enemy_breach: Res<Events<EnemyBreach>>,
    sound: Res<Sounds>, mut audio: ResMut<Audio>
) {
    if enemy_breach_reader.latest(&enemy_breach).is_some() {
        let mut sequence = Sequence::<()>::new(Default::default());
        sequence.play(&sound.breach, Default::default());
        audio.manager.start_sequence(sequence, Default::default()).unwrap();
    }
}

fn break_down_audio(mut audio: ResMut<Audio>) {
    if let Some(background) = &audio.background_handle.clone() {
        audio.manager.remove_arrangement(background).unwrap();
    }
}
