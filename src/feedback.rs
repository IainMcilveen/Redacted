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
                sound: Sounds::Correct,
                setting: PlaybackSettings::ONCE,
            });
            let material = materials.add(StandardMaterial {
                base_color: Color::srgba(0.0, 1.0, 0.0, 1.0), // Green
                alpha_mode: AlphaMode::Blend,
                ..default()
            });

            // We'll reuse a simple Box mesh for both legs, scaled via Transform
            let mesh = meshes.add(Cuboid::new(0.1, 1.0, 0.1));

            // Parent entity with Lifetime
            commands
                .spawn((
                    Transform::from_translation(event.pos).with_scale(Vec3::new(-0.1, 0.1, 0.1)),
                    Visibility::default(),
                    FeedbackObject::new(5.0),
                ))
                .with_children(|parent| {
                    // Short leg (left side)
                    parent.spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(-0.18, -0.1, 0.0) // Lowered slightly
                            .with_rotation(Quat::from_rotation_z(0.785)) // +45 deg (tilted up-right)
                            .with_scale(Vec3::new(1.0, 0.3, 1.0)),
                    ));

                    // Long leg (right side)
                    parent.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(material),
                        Transform::from_xyz(0.16, 0.1, 0.0) // Raised slightly
                            .with_rotation(Quat::from_rotation_z(-0.785)) // -45 deg (tilted up-left)
                            .with_scale(Vec3::new(1.0, 0.68, 1.0)),
                    ));
                });
        }
        Feedbacks::Wrong => {
            commands.trigger(SoundEvent {
                sound: Sounds::Wrong,
                setting: PlaybackSettings::ONCE,
            });
            let mesh = meshes.add(Cuboid::new(0.1, 1.0, 0.1));
            let material = materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.0, 0.0, 1.0),
                alpha_mode: AlphaMode::Blend,
                ..default()
            });

            // Parent entity controls the lifetime and upward drift
            commands
                .spawn((
                    Transform::from_translation(event.pos).with_scale(Vec3::splat(0.1)),
                    Visibility::default(),
                    FeedbackObject::new(3.5),
                ))
                .with_children(|parent| {
                    // Bar 1
                    parent.spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_rotation(Quat::from_rotation_z(0.785)),
                    ));
                    // Bar 2
                    parent.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(material),
                        Transform::from_rotation(Quat::from_rotation_z(-0.785)),
                    ));
                });
        }
    }
}

fn handle_feedback(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut FeedbackObject, &mut Transform, &Children)>,
    child_query: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    for (entity, mut obj, mut transform, children) in &mut query {
        obj.timer.tick(time.delta());
        transform.translation.y += 0.75 * time.delta_secs();

        let alpha = 1.0 - obj.timer.fraction() * 4.0;

        // Update alpha for all children (the bars of the X or checkmark)
        for child in children.iter() {
            if let Ok(material_handle) = child_query.get(child) {
                if let Some(material) = materials.get_mut(material_handle) {
                    material.base_color.set_alpha(alpha);
                }
            }
        }

        if obj.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
