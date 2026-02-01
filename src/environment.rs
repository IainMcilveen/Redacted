use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Affine2;
use bevy::math::ops::floor;
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dPlugin};

use super::GameState;
use crate::loading::GameAssets;
use crate::{CountdownTimer, LIFETIME};

pub const PIXELS_PER_METRE: f32 = 30.0;

#[derive(Component)]
struct GlassCrackWall;

#[derive(Resource)]
struct GlassCrackStage(usize);

#[derive(Resource)]
struct LookingAt(Vec3);

#[derive(Component)]
pub struct Desk;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Sprite3dPlugin)
        .insert_resource(GlassCrackStage(0))
        .insert_resource(LookingAt(Vec3::new(0.0, 1.25, 1.0)))
        .add_systems(OnEnter(GameState::PLAYING), setup)
        .add_systems(Update, (update_glass_cracks, update_looking));
}

fn setup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Sprite::from_image(assets.wall.clone()),
        Sprite3d {
            pixels_per_metre: PIXELS_PER_METRE,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        DespawnOnExit(GameState::PLAYING),
    ));

    commands.spawn((
        Sprite::from_image(assets.glass_cracks[0].clone()),
        Sprite3d {
            pixels_per_metre: PIXELS_PER_METRE,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.001),
        GlassCrackWall,
        DespawnOnExit(GameState::PLAYING),
    ));

    // Desk
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.1, 1.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.4, 0.25, 0.15))),
        Transform::from_xyz(0.0, 0.70, 1.0),
        DespawnOnExit(GameState::PLAYING),
        Desk,
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(GameState::PLAYING),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        // Wall View
        // Transform::from_xyz(0.0, 1.75, -10.0).looking_at(Vec3::new(0.0, 1.75, 1.0), Vec3::Y),
        // Page View
        Transform::from_xyz(0.0, 1.75, 0.0).looking_at(Vec3::new(0.0, 1.25, 1.0), Vec3::Y), //Transform::from_xyz(0.0, 1.0, 3.0).looking_at(Vec3::Y, Vec3::Y),
        DespawnOnExit(GameState::PLAYING),
    ));
}

fn update_glass_cracks(
    timer: ResMut<CountdownTimer>,
    mut glass_crack_stage: ResMut<GlassCrackStage>,
    mut query: Query<&mut Sprite, With<GlassCrackWall>>,
    assets: Res<GameAssets>,
) {
    let progress = timer.0.elapsed_secs() / LIFETIME;
    glass_crack_stage.0 = floor(progress * assets.glass_cracks.len() as f32) as usize;
    for mut sprite in &mut query {
        sprite.image = assets.glass_cracks
            [glass_crack_stage.0.clamp(0, assets.glass_cracks.len() - 1)]
        .clone();
    }
}

fn update_looking(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut looking_at: ResMut<LookingAt>,
    time: Res<Time>,
    mut camera_transform: Single<&mut Transform, With<Camera3d>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        looking_at.0.y += time.delta_secs()
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        looking_at.0.y -= time.delta_secs()
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        looking_at.0.x += time.delta_secs()
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        looking_at.0.x -= time.delta_secs()
    }
    camera_transform.look_at(looking_at.0, Vec3::Y);
}
