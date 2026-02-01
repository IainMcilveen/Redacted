use bevy::{audio::PlaybackMode, platform::collections::HashMap, prelude::*};

use crate::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Sounds {
    VineBoom,
    MarkerDrag,
    Slurp,
    Correct,
    Wrong,
    GlassCrack,
    GlassShatter,
    Mob,
}

#[derive(Event)]
pub struct SoundEvent {
    pub sound: Sounds,
    pub setting: PlaybackSettings,
}

#[derive(Event)]
pub struct StopLoopEvent;

#[derive(Resource, Default)]
pub struct SoundBank {
    pub sounds: HashMap<Sounds, Handle<AudioSource>>,
    pub looping: bool,
}

#[derive(Component)]
struct SoundComponent;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_observer(play_sound)
        .add_observer(stop_loop);
}

// loads all sound effects and inserts resource
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sound_bank = SoundBank::default();

    // load and insert sounds
    sound_bank
        .sounds
        .insert(Sounds::VineBoom, asset_server.load("audio/vine-boom.ogg"));
    sound_bank
        .sounds
        .insert(Sounds::MarkerDrag, asset_server.load("audio/marker.ogg"));
    sound_bank
        .sounds
        .insert(Sounds::Slurp, asset_server.load("audio/slurp.ogg"));
    sound_bank
        .sounds
        .insert(Sounds::Correct, asset_server.load("audio/correct.ogg"));
    sound_bank
        .sounds
        .insert(Sounds::Wrong, asset_server.load("audio/wrong.ogg"));
    sound_bank.sounds.insert(
        Sounds::GlassCrack,
        asset_server.load("audio/glass-crack.ogg"),
    );
    sound_bank.sounds.insert(
        Sounds::GlassShatter,
        asset_server.load("audio/glass-smash.ogg"),
    );
    sound_bank
        .sounds
        .insert(Sounds::Mob, asset_server.load("audio/mob.ogg"));

    commands.insert_resource(sound_bank);
}

fn play_sound(event: On<SoundEvent>, mut commands: Commands, mut sound_bank: ResMut<SoundBank>) {
    if let Some(handle) = sound_bank.sounds.get(&event.sound) {
        commands.spawn((
            AudioPlayer::new(handle.clone()),
            event.setting,
            SoundComponent,
            DespawnOnExit(GameState::PLAYING),
        ));

        match event.setting.mode {
            PlaybackMode::Loop => {
                sound_bank.looping = true;
            }
            _ => {}
        }
    }
}

fn stop_loop(
    _event: On<StopLoopEvent>,
    mut commands: Commands,
    sounds: Query<(Entity, &PlaybackSettings), With<SoundComponent>>,
    mut sound_bank: ResMut<SoundBank>,
) {
    for (entity, settings) in &sounds {
        match settings.mode {
            PlaybackMode::Loop => {
                commands.entity(entity).despawn();
                sound_bank.looping = false;
            }
            _ => {}
        }
    }
}
