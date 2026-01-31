use bevy::{audio::PlaybackMode, platform::collections::HashMap, prelude::*};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Sounds {
    VineBoom,
    MarkerDrag,
}

#[derive(Event)]
pub struct SoundEvent {
    pub sound: Sounds,
    pub setting: PlaybackSettings,
}

#[derive(Event)]
pub struct StopLoopEvent(pub Sounds);

#[derive(Resource, Default, Deref)]
struct SoundBank {
    sounds: HashMap<Sounds, Handle<AudioSource>>,
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

    commands.insert_resource(sound_bank);
}

fn play_sound(event: On<SoundEvent>, mut commands: Commands, sound_bank: Res<SoundBank>) {
    if let Some(handle) = sound_bank.get(&event.sound) {
        commands.spawn((
            AudioPlayer::new(handle.clone()),
            event.setting,
            SoundComponent,
        ));
    }
}

fn stop_loop(
    _event: On<StopLoopEvent>,
    mut commands: Commands,
    sounds: Query<(Entity, &PlaybackSettings), With<SoundComponent>>,
) {
    for (entity, settings) in &sounds {
        match settings.mode {
            PlaybackMode::Loop => {
                commands.entity(entity).despawn();
            }
            _ => {}
        }
    }
}
