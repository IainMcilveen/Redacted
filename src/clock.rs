use std::f32::consts::PI;

use bevy::math::ops::sin;
use bevy::prelude::*;
use bevy::{color::palettes::css, math::ops::floor};
use bevy_rich_text3d::{Text3d, Text3dBounds, Text3dStyling, TextAtlas, Weight};

use crate::CountdownTimer;

const ALARM_CLOCK_MODEL_PATH: &str = "models/alarm_clock.glb";

#[derive(Component)]
struct AlarmClockText;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_mesh)
        .add_systems(Update, update_clock);
}

fn setup_mesh(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let alarm_clock_scene =
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(ALARM_CLOCK_MODEL_PATH)));

    let mut alarm_clock_transform = Transform::from_xyz(0.75, 0.85, 1.5);
    alarm_clock_transform.rotation = Quat::from_rotation_y(225.0 / 360.0 * PI * 2.0);
    alarm_clock_transform.scale = Vec3::splat(0.075);

    commands.spawn((alarm_clock_scene, alarm_clock_transform));

    let mut text_transform = alarm_clock_transform.clone();
    text_transform.rotation = alarm_clock_transform.rotation.clone();
    text_transform.translation -= alarm_clock_transform.forward() * 0.1;

    let mut light_transform = text_transform.clone();
    let mut light_infront = light_transform.clone();
    light_infront.translation -= alarm_clock_transform.forward() * 0.1;

    light_transform.look_at(light_infront.translation, Vec3::Y);

    commands.spawn((
        AlarmClockText,
        Text3d::new(""),
        Text3dBounds { width: 260.0 },
        Text3dStyling {
            font: "monospace".into(),
            weight: Weight::BOLD,
            ..default()
        },
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(css::RED),
            base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
            alpha_mode: AlphaMode::Blend,
            emissive: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
            ..default()
        })),
        text_transform,
        Mesh3d::default(),
    ));
    commands.spawn((
        AlarmClockText,
        SpotLight {
            color: Color::from(css::RED),
            intensity: 7500.0,
            range: 50.0,
            radius: 10.0,
            shadows_enabled: false,
            outer_angle: 90.0 * PI * 2.0 / 360.0,
            ..default()
        },
        light_transform,
    ));
}

fn update_clock(
    timer: Res<CountdownTimer>,
    mut alarm_clock_text: Single<(&mut Text3d, &mut Transform), With<AlarmClockText>>,
    mut alarm_color_light: Single<&mut SpotLight, With<AlarmClockText>>
) {
    let countdown = floor(timer.0.remaining_secs()) as u32;
    let minutes = countdown / 60;
    let seconds = countdown - (minutes * 60);
    alarm_clock_text.0.segments = Text3d::new(format!("{:02}:{:02}", minutes, seconds)).segments;

    let t = timer.0.remaining_secs() - countdown as f32;
    let pulse = sin(t * PI);
    alarm_clock_text.1.scale = Vec3::splat(0.0075 + pulse * 0.001);
    alarm_color_light.intensity = 7500.0 + 7500.0 * pulse;
}
