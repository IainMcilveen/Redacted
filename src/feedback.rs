use bevy::prelude::*;

use crate::{
    GameState,
    audio::{SoundEvent, Sounds},
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Feedbacks {
    Correct,
    Wrong,
}

#[derive(Event)]
pub struct FeedbackEvent {
    pub feedback: Feedbacks,
    pub pos: Vec3,
}

#[derive(Component)]
pub struct FeedbackObject {
    timer: Timer,
}

impl FeedbackObject {
    fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_feedback)
        .add_observer(spawn_feedback);
}

fn spawn_feedback(
    event: On<FeedbackEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // play audio effect
    match event.feedback {
        Feedbacks::Correct => {
            commands.trigger(SoundEvent {
                sound: Sounds::VineBoom,
                setting: PlaybackSettings::ONCE,
            });
        }
        Feedbacks::Wrong => {
            commands.trigger(SoundEvent {
                sound: Sounds::VineBoom,
                setting: PlaybackSettings::ONCE,
            });
        }
    }

    // spawn feedback at pen position
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.0, 0.0, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_translation(event.pos),
        FeedbackObject::new(5.0),
    ));
}

fn handle_feedback(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        Entity,
        &mut FeedbackObject,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
) {
    for (entity, mut obj, mut transform, material_handle) in query.iter_mut() {
        // Tick the timer with delta time
        obj.timer.tick(time.delta());

        // move feedback upwards
        transform.translation.y += 1.0 * time.delta_secs();

        // fade feedback out
        if let Some(material) = materials.get_mut(material_handle) {
            let alpha = 1.0 - obj.timer.fraction(); // Fades from 1.0 to 0.0
            material.base_color.set_alpha(alpha);
        }

        // If finished, despawn the entity
        if obj.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
