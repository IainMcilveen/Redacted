use std::ops::Div;

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Affine2;
use bevy::prelude::*;

use super::GameState;

pub const CAMERA_POS: Vec3 = Vec3::new(0.0, 1.75, 0.0);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::PAGETEST), setup);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut textures: ResMut<Assets<Image>>,
) {
    let wall_texture_handle =
        asset_server.load_with_settings("textures/wall.png", |settings: &mut _| {
            *settings = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,

                    ..default()
                }),
                ..default()
            }
        });

    let wall_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(wall_texture_handle.clone()),
        uv_transform: Affine2::from_scale(Vec2::new(1.0, -1.0)),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // Wall
    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::new(Vec3::NEG_Z, Vec2::new(16.0, 12.0).div(2.0)).mesh())),
    //     MeshMaterial3d(wall_material_handle),
    //     Transform::from_xyz(0.0, 2.0, 10.0),
    // ));

    

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
        Transform::from_xyz(0.0, 1.75, 0.0).looking_at(Vec3::new(0.0, 1.75, 1.0), Vec3::Y), //Transform::from_xyz(0.0, 1.0, 3.0).looking_at(Vec3::Y, Vec3::Y),
    ));
}
