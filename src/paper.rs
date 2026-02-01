use bevy::{
    ecs::event::Trigger,
    picking::events::{Click, Pointer},
    prelude::*,
    scene::SceneInstanceReady,
};

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

use crate::{pen::Marker, text_asset::get_text_file};

const BUTTON_MODEL_PATH: &str = "models/next_button.glb";
pub const BTN_POS: Vec3 = Vec3::new(0.5, 0.78, 1.3);


const MAX_LENGTH: i32 = 24;
const MAX_HEIGHT: i32 = 25;

use crate::paint::ClearEvent;

use super::GameState;
struct PageText {
    pages: Vec<String>,
}

#[derive(Component)]
struct GoNextPage {
    go: bool,
    can_go: bool,
}

// #[derive(Component)]
// struct AnimationToPlay {
//     graph_handle: Handle<AnimationGraph>,
//     index: AnimationNodeIndex,
// }

// fn play_animation_when_ready(
//     scene_ready: On<SceneInstanceReady>,
//     mut commands: Commands,
//     children: Query<&Children>,
//     animations_to_play: Query<&AnimationToPlay>,
//     mut players: Query<&mut AnimationPlayer>,
// ) {
//     // The entity we spawned in `setup_mesh_and_animation` is the trigger's target.
//     // Start by finding the AnimationToPlay component we added to that entity.
//     if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
//         // The SceneRoot component will have spawned the scene as a hierarchy
//         // of entities parented to our entity. Since the asset contained a skinned
//         // mesh and animations, it will also have spawned an animation player
//         // component. Search our entity's descendants to find the animation player.
//         for child in children.iter_descendants(scene_ready.entity) {
//             if let Ok(mut player) = players.get_mut(child) {
//                 // Tell the animation player to start the animation and keep
//                 // repeating it.
//                 //
//                 // If you want to try stopping and switching animations, see the
//                 // `animated_mesh_control.rs` example.
//                 println!("REPEATING");
//                 player.play(animation_to_play.index).repeat();

//                 // Add the animation graph. This only needs to be done once to
//                 // connect the animation player to the mesh.
//                 commands
//                     .entity(child)
//                     .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
//             }
//         }
//     }
// }

// fn setup_animation(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut graphs: ResMut<Assets<AnimationGraph>>,
// ){
//     let (graph, index) = AnimationGraph::from_clip(
//         asset_server.load(GltfAssetLabel::Animation(0).from_asset(BUTTON_MODEL_PATH))
//     );

//         let graph_handle = graphs.add(graph);

//     // Create a component that stores a reference to our animation.
//     let animation_to_play = AnimationToPlay {
//         graph_handle,
//         index,
//     };
//         let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(BUTTON_MODEL_PATH)));

//     // Spawn an entity with our components, and connect it to an observer that
//     // will trigger when the scene is loaded and spawned.
//     commands
//         .spawn((animation_to_play, mesh_scene, GoNextPage{go: true, can_go: true}, Transform::from_translation(BTN_POS).with_scale(Vec3::splat(0.1)).with_rotation(Quat::from_rotation_y(3.14159))))
//         .observe(play_animation_when_ready);
// }

impl PageText {
    pub fn from_text(full_text: String) -> Self {
        let mut pages: Vec<String> = Vec::new();

        let mut row = 0;
        let mut col: i32 = 0;
        let mut total_chars = 0;
        let mut to_redact = false;
        let mut current_page_string = String::new();
        for word in full_text.split(" ") {
            if row > MAX_HEIGHT {
                pages.push(current_page_string.clone());
                current_page_string = String::new();
                row = 0;
                col = 0
            }
            let word_string = word.to_string();
            if col + word_string.len() as i32 > MAX_LENGTH {
                row += 1;
                col = 0;
            }
            for c in word_string.chars() {
                if c == '<' {
                    current_page_string.push(c);
                    to_redact = true;
                    continue;
                } else if c == '>' {
                    current_page_string.push(c);
                    to_redact = false;
                    continue;
                }
                current_page_string.push(c);

                total_chars += 1;
                col += 1;
            }
            current_page_string.push(' ');
            col += 1;
        }
        Self { pages: pages }
    }
}

#[derive(Component)]
pub struct Page {
    pages: PageText,
    pub to_redact: u32,
    pub is_redacted: u32,
    pub total_chars: u32,
    pub page_num: i32,
}

#[derive(Resource, Default)]
pub struct PageScores {
    pub total_chars: u32,
    pub total_to_redact: u32,
    pub correctly_redacted: u32,
}

// #[derive(Resource)]
// pub struct Pages{
//     pages: Vec<String>
// }

#[derive(Component, Debug)]
pub struct Character {
    pub to_redact: bool,
    pub is_redacted: bool,
    pub page_num: u32,
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
    .insert_resource(PageScores {
        total_chars: 0,
        total_to_redact: 0,
        correctly_redacted: 0,
    })
    // .add_systems(Startup, setup_animation)
    .add_systems(OnEnter(GameState::PLAYING), (setup))
    .add_systems(
        FixedUpdate,
        check_redacted.run_if(in_state(GameState::PLAYING)),
    )
    .add_systems(Update, check_button.run_if(in_state(GameState::PLAYING)))
    .add_systems(FixedUpdate, next_page);
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

fn next_page(
    mut commands: Commands,
    chars: Query<(&Character, Entity)>,
    mut page: Single<&mut Page>,
    mut go_next_page: Single<&mut GoNextPage>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut page_scores: ResMut<PageScores>,
) {
    if go_next_page.go {
        go_next_page.go = false;
        for (c, ent) in chars.iter() {
            commands.entity(ent).despawn();
        }
        commands.trigger(ClearEvent);
        // let page_string = "That's all the family news that we're allowed to talk about. We really hope you'll come and visit us soon. I mean we're literally begging you to visit us. And make it quick before they <kill us> Now it's time for Christmas dinner - I think the robots sent us a pie! You know I love my soylent green.";
        // let page_string = get_text_file("assets/text/beemovie.txt") .expect("CAN't LOAD BEE MOVIE");

        let mut batch_spawn: Vec<(
            Text3d,
            Text3dBounds,
            Text3dStyling,
            MeshMaterial3d<StandardMaterial>,
            Transform,
            Mesh3d,
            Character,
            DespawnOnExit<GameState>,
        )> = Vec::new();
        page.page_num += 1;
        let x_offset = 0.022;
        let y_offset = 0.032;
        let mut row = 0;
        let mut col: i32 = 0;
        let mut to_redact = false;
        let mut total_to_redact = 0;
        let mut total_chars = 0;
        let mut chars_skipped: u32 = 0;
        // If this line fails it means we have reached the end of the pages
        let page_string = page
            .pages
            .pages
            .get(page.page_num as usize)
            .expect("Can't get page at index");
        for word in page_string.split(" ") {
            if row > MAX_HEIGHT {
                break;
            }
            let word_string = word.to_string();
            if col + word_string.len() as i32 > MAX_LENGTH {
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
                batch_spawn.push((
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
                    .with_scale(Vec3::splat(0.0025)),
                    Mesh3d::default(),
                    Character {
                        to_redact: to_redact,
                        is_redacted: false,
                        page_num: page.page_num as u32,
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
        page.total_chars += total_chars;
        page.to_redact += total_to_redact;
        commands.spawn_batch(batch_spawn);

        // update page score resource
        page_scores.total_chars += total_chars;
        page_scores.total_to_redact += total_to_redact;
    }
}

fn check_button(
    marker: Single<&Marker>,
    mut go_next_page: Single<&mut GoNextPage>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    // mut players: Query<&mut AnimationPlayer>,
    // children: Query<&Children>,
    // ani_to_play: Single<(&AnimationToPlay, Entity)>
) {
    // let marker = marker
    if let Some(tip_location) = marker.tip_location {
        if tip_location.distance(BTN_POS) < 0.095 && go_next_page.can_go {
            println!("NEXT_PAGE");
            go_next_page.go = true;
            go_next_page.can_go = false;
        //    let (to_play, ent) = ani_to_play.into_inner();
        //     for child in children.iter_descendants(ent) {
        //         if let Ok(mut player) = players.get_mut(child) {
        //             // Tell the animation player to start the animation and keep
        //             // repeating it.
        //             //
        //             // If you want to try stopping and switching animations, see the
        //             // `animated_mesh_control.rs` example.
        //             println!("TRYING TO PLAY");
        //             player.play(to_play.index).repeat();
        //         }
        //     }
        } else {
            go_next_page.go = false
        }
    }
    if mouse_btn.just_released(MouseButton::Left) {
        go_next_page.can_go = true;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut page_scores: ResMut<PageScores>,
) {
    let ink_mesh_scene =
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(BUTTON_MODEL_PATH)));

    commands.spawn((
        ink_mesh_scene,
        GoNextPage {
            go: true,
            can_go: true,
        },
        Transform::from_translation(BTN_POS)
            .with_scale(Vec3::splat(0.1))
            .with_rotation(Quat::from_rotation_y(3.14159)),
        DespawnOnExit(GameState::PLAYING),
    ));

    // Text on the paper
    // let page_string = "That's all the family news that we're allowed to talk about. We really hope you'll come and visit us soon. I mean we're literally begging you to visit us. And make it quick before they <kill us> Now it's time for Christmas dinner - I think the robots sent us a pie! You know I love my soylent green.";
    let page_string = get_text_file("assets/text/beemovie.txt").expect("CAN't LOAD BEE MOVIE");

    // Paper
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(0.6, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(PAPER_POS),
        Page {
            pages: PageText::from_text(page_string.into()),
            is_redacted: 0,
            to_redact: 0,
            total_chars: 0,
            page_num: -1,
        },
        DespawnOnExit(GameState::PLAYING),
    ));

    // reset score state
    page_scores.correctly_redacted = 0;
    page_scores.total_chars = 0;
    page_scores.total_to_redact = 0;

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
