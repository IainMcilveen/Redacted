use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dPlugin};

use super::GameState;
use crate::loading::GameAssets;

#[derive(Component)]
struct GlassCrackWall;

#[derive(Resource)]
struct GlassCrackStage(usize);

#[derive(Resource)]
struct LookingAt(Vec3);

#[derive(Resource)]
struct TemporaryTimer(Timer);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Sprite3dPlugin)
        .add_systems(OnEnter(GameState::PAGETEST), setup)
        .add_systems(Update, update_glass_cracks)
        .add_systems(Update, update_looking);

    app.insert_resource(GlassCrackStage(0));
    app.insert_resource(LookingAt(Vec3::new(0.0, 1.25, 1.0)));

    app.insert_resource(TemporaryTimer(Timer::from_seconds(
        0.5,
        TimerMode::Repeating,
    )));
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
            pixels_per_metre: 40.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    commands.spawn((
        Sprite::from_image(assets.glass_cracks[0].clone()),
        Sprite3d {
            pixels_per_metre: 40.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        GlassCrackWall,
    ));

    // Desk
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.1, 1.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.4, 0.25, 0.15))),
        Transform::from_xyz(0.0, 0.70, 1.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        // Wall View
        // Transform::from_xyz(0.0, 1.75, -10.0).looking_at(Vec3::new(0.0, 1.75, 1.0), Vec3::Y),
        // Page View
        Transform::from_xyz(0.0, 1.75, 0.0).looking_at(Vec3::new(0.0, 1.25, 1.0), Vec3::Y), //Transform::from_xyz(0.0, 1.0, 3.0).looking_at(Vec3::Y, Vec3::Y),
    ));
}

fn update_glass_cracks(
    time: Res<Time>,
    mut timer: ResMut<TemporaryTimer>,
    mut glass_crack_stage: ResMut<GlassCrackStage>,
    mut query: Query<&mut Sprite, With<GlassCrackWall>>,
    assets: Res<GameAssets>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        glass_crack_stage.0 = (glass_crack_stage.0 + 1) % 11;
    }
    for mut sprite in &mut query {
        sprite.image = assets.glass_cracks[glass_crack_stage.0].clone();
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
