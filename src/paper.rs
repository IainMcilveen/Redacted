use bevy::{ecs::event::Trigger,picking::events::Click, picking::events::Pointer ,prelude::*};

use bevy_rich_text3d::{
    //TouchTextMaterial3dPlugin, // Required for dynamic text updates
    LoadFonts, Text3d, Text3dBounds, Text3dPlugin, Text3dStyling, TextAtlas, Weight
};

use super::GameState;

#[derive(Default)]
struct Page {
    text: String,
}

#[derive(Component)]
pub struct Character(pub bool);



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
    let text = String::from("Hello World The quick brown fox jumped over the lazy dog. Well you found me, was it worth it, because depsite your violent behaviour the only thing you've managed to break so far is my heart.");
    let x_offset = 0.022;
    let y_offset = 0.032;
    let mut row = 0;
    let mut col = 0;
    let max_length = 25;
    for word in text.split(" ") {
        let word_string = word.to_string();
        if col + word_string.len() > max_length{
            row += 1;
            col = 0;
        }

        for c in word_string.chars() {
            commands.spawn((
                Text3d::new(c),
                Text3dBounds { width: 260.0 },
                Text3dStyling {
                    font: "monospace".into(),
                    weight: Weight::BOLD,
                    ..default()
                },
                MeshMaterial3d(materials.add(StandardMaterial {
                    // Use the shared texture atlas for efficient rendering
                    base_color: Color::BLACK,
                    base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
                Transform::from_translation(((PAPER_POS + Vec3{x: 0.25, y: 0.0, z: 0.4}) + Vec3::Y * 0.001 - (Vec3{x: x_offset*col as f32, y: 0.0, z: y_offset*row as f32}))) 
                    .with_rotation(
                        Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                            * Quat::from_rotation_z(std::f32::consts::PI),
                    )
                    .with_scale(Vec3::splat(0.0022)),
                Mesh3d::default(),
                Character(c == 'e')
            ));
            col += 1;
        }
        col += 1;


    }

    // commands.spawn((
    //     Text3d::new("123456789098765432123456789098765 In accordance with the determinations reached during the most recent closed procedural interval, all affected parties are advised that preliminary conditions have now been satisfied and that subsequent measures will proceed without further notice.\n\nAny variance from the established sequence, whether intentional or incidental, will be documented and reconciled under the appropriate review instruments. Stakeholders should consider this communication to constitute sufficient advisory of impending adjustments, the full scope of which will be disclosed only upon completion of the requisite confirmations."),
    //     Text3dBounds { width: 260.0 },
    //     Text3dStyling {
    //         font: "monospace".into(),
    //         weight: Weight::BOLD,
    //         ..default()
    //     },
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         // Use the shared texture atlas for efficient rendering
    //         base_color: Color::BLACK,
    //         base_color_texture: Some(TextAtlas::DEFAULT_IMAGE.clone()),
    //         alpha_mode: AlphaMode::Blend,
    //         ..default()
    //     })),
    //     Transform::from_translation(PAPER_POS + Vec3::Y * 0.001) 
    //         .with_rotation(
    //             Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
    //                 * Quat::from_rotation_z(std::f32::consts::PI),
    //         )
    //         .with_scale(Vec3::splat(0.0022)),
    //     Mesh3d::default(),
    // ));

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
