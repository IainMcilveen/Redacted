use bevy::{math::VectorSpace, prelude::*};

use bevy_rich_text3d::{
    //TouchTextMaterial3dPlugin, // Required for dynamic text updates
    LoadFonts,
    Text3d,
    Text3dBounds,
    Text3dPlugin,
    TextAtlas,
};

use super::GameState;

#[derive(Default)]
struct Page {
    text: String,
}

#[derive(Resource, Default)]
struct Game {
    page: Page,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Text3dPlugin {
        load_system_fonts: true,
        ..Default::default()
    })
    // .add_plugins(TouchTextMaterial3dPlugin)
    .insert_resource(LoadFonts {
        font_paths: vec!["assets/fonts/SpaceMono-Regular.ttf".to_owned()],
        ..default()
    })
    .add_systems(OnEnter(GameState::PAGETEST), setup);
    // .add_systems(
    //     Update,
    //     (menu_action, button_system).run_if(in_state(GameState::MENU)),
    // );
}

const PAPER_POS: Vec3 = Vec3::new(0.0, 0.8, 1.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(25.0, 25.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.8, 0.9))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // // Desk
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.1, 1.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.4, 0.25, 0.15))),
        Transform::from_xyz(0.0, 0.70, 1.0),
    ));

    // Paper
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(0.6, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(PAPER_POS),
    ));

    // Text on the paper
    // commands.spawn((
    //     //Text::new("In accordance with the determinations reached during the most recent closed procedural interval, all affected parties are advised that preliminary conditions have now been satisfied and that subsequent measures will proceed without further notice. Any variance from the established sequence, whether intentional or incidental, will be documented and reconciled under the appropriate review instruments. Stakeholders should consider this communication to constitute sufficient advisory of impending adjustments, the full scope of which will be disclosed only upon completion of the requisite confirmations."),
    //     Text::new("In accordance with the determinations"),
    //     TextFont {
    //         font_size: 20.0, // Adjust size to fit your 0.6x1.0 paper
    //         ..default()
    //     },
    //     TextColor(Color::BLACK),
    //     Transform::from_translation(PAPER_POS + Vec3::Y * 0.01)
    //         .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
    //         .with_scale(Vec3::splat(0.01)), // Slight lift to sit on top
    // ));
    commands.spawn((
        Text3d::new("In accordance with the determinations reached during the most recent closed procedural interval, all affected parties are advised that preliminary conditions have now been satisfied and that subsequent measures will proceed without further notice.\n\nAny variance from the established sequence, whether intentional or incidental, will be documented and reconciled under the appropriate review instruments. Stakeholders should consider this communication to constitute sufficient advisory of impending adjustments, the full scope of which will be disclosed only upon completion of the requisite confirmations."),
        Text3dBounds { width: 220.0 },
        MeshMaterial3d(materials.add(StandardMaterial {
            // Use the shared texture atlas for efficient rendering
            base_color: Color::BLACK,
            base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        //Transform::from_translation(PAPER_POS + Vec3::Y * 0.01) // Lift 1mm
        //     .with_scale(Vec3::splat(0.01)),
        Transform::from_translation(PAPER_POS + Vec3::Y * 0.01) // Lift 1mm
            .with_rotation(
                Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                    * Quat::from_rotation_z(std::f32::consts::PI),
            )
            .with_scale(Vec3::splat(0.0025)),
        Mesh3d::default(),
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
        Transform::from_xyz(0.0, 1.75, 0.0).looking_at(PAPER_POS, Vec3::Y),
        //Transform::from_xyz(0.0, 1.0, 3.0).looking_at(Vec3::Y, Vec3::Y),
    ));
}
