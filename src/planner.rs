use bevy::color::palettes::css;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::math::ops::floor;
use bevy::math::{Affine2, VectorSpace};
use bevy::prelude::*;
use bevy_rich_text3d::{Text3d, Text3dBounds, Text3dStyling, TextAtlas, Weight};
use bevy_sprite3d::{Sprite3d, Sprite3dPlugin};

use super::GameState;
use crate::loading::GameAssets;
use crate::paper::{Page, PageScores};
use crate::{CountdownTimer, LIFETIME};

// pub const PLANNER_POS: Vec3 = Vec3::new(0.65, 0.78, 0.9);
pub const BOSS_POS: Vec3 = Vec3::new(-4.0, 0.5, 7.0);
pub const PLANNER_POS: Vec3 = Vec3::new(-3.0, 2.0, 7.0);
const BOSS_MODEL_PATH: &str = "models/boss.glb";

#[derive(Component)]
struct Planner;

#[derive(Component)]
struct Boss;

#[derive(Component)]
struct PlannerText;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::PLAYING), setup)
        .add_systems(FixedUpdate, update_scores);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::new(Vec3::NEG_Z, Vec2::splat(0.5))
                    .mesh()
                    .size(1.9, 1.5),
            ),
        ),
        // MeshMaterial3d(materials.add(Color::WHITE)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(css::WHITE),
            // base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
            alpha_mode: AlphaMode::Blend,
            emissive: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
            ..default()
        })),
        Transform::from_translation(PLANNER_POS - Vec3::new(0.0, 0.001, -0.001)),
        Planner,
        DespawnOnExit(GameState::PLAYING),
    ));
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::new(Vec3::NEG_Z, Vec2::splat(0.5))
                    .mesh()
                    .size(1.95, 1.55),
            ),
        ),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_translation(PLANNER_POS - Vec3::new(0.0, 0.001, -0.002)),
        Planner,
        DespawnOnExit(GameState::PLAYING),
    ));

    commands.spawn((
        PlannerText,
        Text3d::new("SCORE"),
        Text3dBounds { width: 200.0 },
        Text3dStyling {
            font: "monospace".into(),
            weight: Weight::BOLD,
            ..default()
        },
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(css::BLACK),
            base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
            alpha_mode: AlphaMode::Blend,
            emissive: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            ..default()
        })),
        Mesh3d::default(),
        Transform::from_translation(PLANNER_POS)
            .with_scale(Vec3::splat(0.01))
            .with_rotation(Quat::from_rotation_x(3.14) * Quat::from_rotation_z(3.14)),
        // Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
        //     * Quat::from_rotation_z(std::f32::consts::PI),),
        DespawnOnExit(GameState::PLAYING),
    ));

    let ink_mesh_scene =
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(BOSS_MODEL_PATH)));
    commands.spawn((
        Boss,
        ink_mesh_scene,
        Transform::from_scale(Vec3::new(0.5, 0.5, 0.5))
            .with_translation(BOSS_POS)
            .with_rotation(Quat::from_rotation_y(2.8)),
        DespawnOnExit(GameState::PLAYING),
    ));
}

fn update_scores(
    score_res: Res<PageScores>,
    mut text3d: Single<&mut Text3d, With<PlannerText>>,
    page: Single<&Page>,
) {
    if score_res.is_changed() {
        let correct_redacted = score_res.page_redaction;
        let unredacted = score_res.page_total - correct_redacted;

        text3d.segments = Text3d::new(format!("Boss:\n\nRedact anything related to Bees NOW\n\nPage {}/{}\nRedacted: {}\nUnredacted: {}",
        page.page_num + 1,
        page.pages.pages.len(),
        correct_redacted,
        unredacted)
     ).segments;
    }
}
