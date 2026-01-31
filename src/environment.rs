use std::f32::consts::PI;
use std::ops::Div;

use bevy::asset::LoadedFolder;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Affine2;
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dPlugin};

use super::GameState;
use crate::loading::GlassCracksFolder;

pub const CAMERA_POS: Vec3 = Vec3::new(0.0, 1.75, 0.0);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Sprite3dPlugin)
        .add_systems(OnEnter(GameState::PAGETEST), setup);
}

fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
    println!("Creating texture atlas!");
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    texture_atlas_builder.padding(padding.unwrap_or_default());
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };
        println!("Some Texture: {id}");
        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let result = texture_atlas_builder.build();
    if result.is_err() {
        let error_result = result.err().unwrap();
        println!("Error {error_result}")
    }

    let (texture_atlas_layout, texture_atlas_sources, texture) =
        texture_atlas_builder.build().unwrap();

    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture_atlas_sources, texture)
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load multiple, individual sprites from a folder
    // commands.insert_resource(RpgSpriteFolder(asset_server.load_folder("textures/rpg")));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    glass_cracks_handles: Res<GlassCracksFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let (glass_cracks_layout, _sources, atlas_image) = create_texture_atlas(
        loaded_folders.get(&glass_cracks_handles.0).unwrap(),
        Some(UVec2::splat(2)),
        None,
        &mut textures,
    );

    let glass_cracks_layout_handle = texture_atlas_layouts.add(glass_cracks_layout);

    // let glass_cracks_material_handle = materials.add(StandardMaterial {
    //     base_color_texture: Some(atlas_image.clone()),
    //     unlit: true,
    //     ..default()
    // });

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

    commands.spawn((
        Sprite::from_atlas_image(
            atlas_image,
            TextureAtlas {
                layout: glass_cracks_layout_handle,
                index: 0,
            },
        ),
        Sprite3d {
            pixels_per_metre: 40.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            // pivot: Some(Vec2::new(0.5, 0.5)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    // Glass Cracks
    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::new(Vec3::NEG_Z, Vec2::new(16.0, 12.0).div(2.0)).mesh())),
    //     MeshMaterial3d(glass_cracks_material_handle),
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
