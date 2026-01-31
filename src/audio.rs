use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Sounds {
    VineBoom,
}

#[derive(Event)]
pub struct SoundEvent {
    sound: Sounds,
}

#[derive(Resource, Default, Deref)]
struct SoundBank {
    sounds: HashMap<Sounds, Handle<AudioSource>>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup).add_observer(play_sound);
}

// loads all sound effects and inserts resource
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sound_bank = SoundBank::default();

    sound_bank
        .sounds
        .insert(Sounds::VineBoom, asset_server.load("audio/vine-boom.mp3"));

    commands.insert_resource(sound_bank);
}

fn play_sound(event: On<SoundEvent>, mut commands: Commands, sound_bank: Res<SoundBank>) {
    if let Some(handle) = sound_bank.sounds.get(&event.sound) {
        commands.spawn(AudioPlayer::new(handle.clone()));
    }
}
