use bevy::{ecs::event::Trigger, picking::events::Click, picking::events::Pointer, prelude::*};

use bevy_rich_text3d::{
    //TouchTextMaterial3dPlugin, // Required for dynamic text updates
    LoadFonts,
    Text3d,
    Text3dBounds,
    Text3dPlugin,
    Text3dStyling,
    TextAtlas,
    Weight,
};

use super::GameState;

#[derive(Component)]
pub struct Page {
    text: String,
    pub to_redact: u32,
    pub is_redacted: u32,
    pub total_chars: u32,
}

#[derive(Component, Debug)]
pub struct Character {
    pub to_redact: bool,
    pub is_redacted: bool,
}

#[derive(Resource, Default)]
pub struct Game {
    // pub pages: Vec<Page>,
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
    .add_systems(OnEnter(GameState::PLAYING), setup)
    .add_systems(FixedUpdate, check_redacted);
    // .add_systems(
    //     Update,
    //     (menu_action, button_system).run_if(in_state(GameState::MENU)),
    // );
}

fn check_redacted(page_q: Query<&Character>) {
    let total_redacted: i32 = page_q
        .iter()
        .map(|item| if item.is_redacted { 1 } else { 0 })
        .sum();
    let to_redact: i32 = page_q
        .iter()
        .map(|item| {
            if item.to_redact & !item.is_redacted {
                1
            } else {
                0
            }
        })
        .sum();
    // for character in page_q.iter() {

    // }
    // println!("is_redacted: {}, to_redact: {}", total_redacted, to_redact);
}

pub const PAPER_POS: Vec3 = Vec3::new(0.0, 0.8, 1.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Text on the paper
    let page_string = "That's all the family news that we're allowed to talk about. We really hope you'll come and visit us soon. I mean we're literally begging you to visit us. And make it quick before they <kill us> Now it's time for Christmas dinner - I think the robots sent us a pie! You know I love my soylent green.";

    let x_offset = 0.022;
    let y_offset = 0.032;
    let mut row = 0;
    let mut col = 0;
    let max_length = 25;
    let mut to_redact = false;
    let mut total_to_redact = 0;
    let mut total_chars = 0;
    for word in page_string.split(" ") {
        let word_string = word.to_string();
        if col + word_string.len() > max_length {
            row += 1;
            col = 0;
        }

        for c in word_string.chars() {
            if c == '<' {
                to_redact = true;
                continue;
            } else if c == '>' {
                to_redact = false;
                continue;
            }
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
                Transform::from_translation(
                    ((PAPER_POS
                        + Vec3 {
                            x: 0.25,
                            y: 0.0,
                            z: 0.4,
                        })
                        + Vec3::Y * 0.001
                        - (Vec3 {
                            x: x_offset * col as f32,
                            y: 0.0,
                            z: y_offset * row as f32,
                        })),
                )
                .with_rotation(
                    Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                        * Quat::from_rotation_z(std::f32::consts::PI),
                )
                .with_scale(Vec3::splat(0.0022)),
                Mesh3d::default(),
                Character {
                    to_redact: to_redact,
                    is_redacted: false,
                },
                DespawnOnExit(GameState::PLAYING),
            ));
            if to_redact {
                total_to_redact += 1;
            }
            total_chars += 1;
            col += 1;
        }
        col += 1;
    }

    // Paper
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(0.6, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(PAPER_POS),
        Page {
            is_redacted: 0,
            to_redact: total_to_redact,
            total_chars: total_chars,
            text: page_string.into(),
        },
        DespawnOnExit(GameState::PLAYING),
    ));

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
}
