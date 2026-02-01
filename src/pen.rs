use bevy_window::{CursorGrabMode, CursorOptions, Window};
use std::{f32::consts::PI, time::Duration};

use bevy::{
    color::palettes::css, input::mouse::AccumulatedMouseMotion, light::CascadeShadowConfigBuilder,
    prelude::*, scene::SceneInstanceReady, time::common_conditions::once_after_real_delay,
};

use crate::{
    audio::{SoundEvent, Sounds, StopLoopEvent},
    feedback::{FeedbackEvent, Feedbacks},
    paint::PaintPlane,
    paper::Character,
};

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app
        // .add_systems(Startup, set_mouse_setting)
        .add_systems(Startup, setup_mesh_and_animation)
        .add_systems(Startup, set_mouse_setting)
        .add_systems(Startup, create_ink_meter)
        .add_systems(Update, mouse_motion_system)
        .add_systems(Update, marker_animation_change)
        .add_systems(Update, setup_scene_once_loaded)
        .add_systems(Update, update_ink_supply_meter)
        .add_systems(Update, (pen_drop, ray_cast_system))
        .add_systems(FixedUpdate, can_draw_check)
        .add_systems(FixedUpdate, check_refill);

    }

// An example asset that contains a mesh and animation.
const GLTF_PATH: &str = "models/marker_2.glb";
const INK_MODEL_PATH: &str = "models/ink_res_dev.glb";
pub const INK_RES_POS: Vec3 = Vec3::new(-0.5, 0.8, 1.5);

// A component that stores a reference to an animation we want to play. This is
// created when we start loading the mesh (see `setup_mesh_and_animation`) and
// read when the mesh has spawned (see `play_animation_once_loaded`).

#[derive(Resource)]
struct PenAnimations {
    animations: Vec<AnimationNodeIndex>,
    current_annimation: usize,
    graph_handle: Handle<AnimationGraph>,
}

#[derive(Component)]
pub struct InkSupplyPercent(pub f32, pub bool);

#[derive(Component)]
struct InkSupplyMeter();

#[derive(Component, Default, Clone, Copy)]
pub struct Marker {
    pub tip_location: Option<Vec3>,
    pub can_draw: bool
}

#[derive(Component)]
struct InkRes;

fn create_ink_meter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        InkSupplyMeter(),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(css::YELLOW))),
        Transform::from_xyz(0.0, 1.0, 1.0).with_scale(Vec3::splat(0.1)),
    ));
}

fn setup_mesh_and_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create an animation graph containing a single animation. We want the "run"
    // animation from our example asset, which has an index of two.
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GLTF_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(GLTF_PATH)),
    ]);

    // Store the animation graph as an asset.

    let graph_handle = graphs.add(graph);
    commands.insert_resource(PenAnimations {
        animations: node_indices,
        current_annimation: 1,
        graph_handle,
    });

    // Start loading the asset as a scene and store a reference to it in a
    // SceneRoot component. This component will automatically spawn a scene
    // containing our mesh once it has loaded.
    let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)));
    let ink_mesh_scene =
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(INK_MODEL_PATH)));

    // Spawn an entity with our components, and connect it to an observer that
    // will trigger when the scene is loaded and spawned.
    commands.spawn((
        Marker::default(),
        InkSupplyPercent(100.0, false),
        mesh_scene,
        Transform::from_scale(Vec3::splat(0.03))
            .with_rotation(Quat::from_rotation_z(0.5))
            .with_translation(Vec3::new(0.0, 1.1, 1.0)),
    ));
    // INK RES
    commands.spawn((
        InkRes,
        ink_mesh_scene,
        Transform::from_scale(Vec3::new(0.1, 0.05, 0.1)).with_translation(INK_RES_POS),
    ));
}

fn ray_cast_system(
    mut commands: Commands,
    mut raycast: MeshRayCast,
    mut pen_q: Single<(&Transform, &mut Marker), With<Marker>>,
    mut q: Query<&mut Character>,
    ignore_q: Query<Entity, With<PaintPlane>>,
    mut gizmos: Gizmos,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    // marker query
    let pen_transform = pen_q.0;
    let mut marker = pen_q.1.reborrow();

    // Only check for redacts when pressing left mouse button
    // Otherwise clear tip location
    if !mouse.pressed(MouseButton::Left) {
        marker.tip_location = None;
        return;
    }

    // setup ray cast with marker rotation
    // make sure that the drawing plane gets filtered out
    let rot = Quat::from_rotation_z(0.5);
    let dir_vec = rot * Vec3::NEG_Y;
    let ray = Ray3d::new(pen_transform.translation, Dir3::new(dir_vec).unwrap());
    let filter = |entity| !ignore_q.contains(entity);
    let settings = MeshRayCastSettings::default().with_filter(&filter);
    let hits = raycast.cast_ray(ray, &settings);
    gizmos.line(ray.origin, ray.origin + dir_vec, Color::from(css::RED));

    for (ent, ray_mesh_hit) in hits {
        println!("{:?}", ent);

        // update marker tip location for painting
        marker.tip_location = Some(ray_mesh_hit.point);

        if let Ok(mut character) = q.get_mut(*ent) {
            if character.to_redact {
                character.is_redacted = true;

                match marker.tip_location {
                    Some(pos) => {
                        commands.trigger(FeedbackEvent {
                            feedback: Feedbacks::Correct,
                            pos: pos,
                        });
                    }
                    _ => {}
                }
            }
            // println!("redacted?, {}", character.to_redact);
        }
        // println!("{:?}", hits);
    }
}

fn set_mouse_setting(mut windows: Query<(&Window, &mut CursorOptions)>) {
    for (window, mut cursor_options) in &mut windows {
        if !window.focused {
            continue;
        }

        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    }
}

fn pen_drop(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mut pen: Single<&mut Transform, With<Marker>>,
) {
    // start looping audio if pen dropped
    if mouse.just_pressed(MouseButton::Left) {
        commands.trigger(SoundEvent {
            sound: Sounds::MarkerDrag,
            setting: PlaybackSettings::LOOP,
        });
    }

    // stop looping audio if pen raised
    if mouse.just_released(MouseButton::Left) {
        commands.trigger(StopLoopEvent(Sounds::MarkerDrag));
    }

    if mouse.pressed(MouseButton::Left) {
        pen.translation.y = 0.988;
    } else {
        pen.translation.y = 1.1;
    }
}

fn marker_animation_change(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut animations: ResMut<PenAnimations>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
            continue;
        };
        if keyboard_input.just_pressed(KeyCode::Enter) {
            // println!("Change?");
            animations.current_annimation =
                (animations.current_annimation + 1) % animations.animations.len();

            transitions.play(
                &mut player,
                animations.animations[animations.current_annimation],
                Duration::from_millis(1),
            );
        }
    }
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<PenAnimations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions.play(&mut player, animations.animations[1], Duration::ZERO);

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph_handle.clone()))
            .insert(transitions);
    }
}

fn update_ink_supply_meter(
    mut transform: Single<&mut Transform, With<InkSupplyMeter>>,
    ink_supply: Single<&InkSupplyPercent>,
    pen_trans: Single<&Transform, (With<Marker>, Without<InkSupplyMeter>)>,
) {
    let meter_scale = ink_supply.0 * 0.3 / 100.0;
    let final_trans = Vec3 {
        x: 0.0,
        y: transform.scale.y / 2.0,
        z: 0.0,
    } + pen_trans.translation
        + (Vec3::X * 0.2);
    transform.translation = final_trans;
    transform.scale.y = meter_scale
}

fn check_refill(marker_q: Single<(&Marker, &mut InkSupplyPercent)>) {
    let (marker, mut ink_supply) = marker_q.into_inner();
    if let Some(tip_location) = marker.tip_location {
        if tip_location.distance(INK_RES_POS) < 0.08 {
            ink_supply.0 += 1.0;
            ink_supply.1 = true;
        } else {
            ink_supply.1 = false;
        }
    }
}

fn can_draw_check(mut single: Single<(&mut Marker, &InkSupplyPercent)>, pen_anim: Res<PenAnimations>){
    let (mut marker, ink_sup) = single.into_inner();
    if ink_sup.0 <= 0.0 {
        marker.can_draw = false;
    }
    else if(pen_anim.current_annimation == 0){
        marker.can_draw = false
    }
    else{
        marker.can_draw = true;
    }

}

fn mouse_motion_system(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut marker: Single<&mut Transform, With<Marker>>,
) {
    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        // println!("{:?}", delta);
        marker.translation += Vec3 {
            x: -delta.x / 400.0,
            y: 0.0,
            z: -delta.y / 400.0,
        };
        // println!("{:?}", marker.translation);
    }
}
