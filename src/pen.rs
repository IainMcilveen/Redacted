use bevy_window::{CursorGrabMode, CursorOptions, Window};
use std::f32::consts::PI;

use bevy::{
    input::mouse::AccumulatedMouseMotion, light::CascadeShadowConfigBuilder, prelude::*,
    scene::SceneInstanceReady,
    color::palettes::css
};

use crate::paper::Character;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app
        // .add_systems(Startup, set_mouse_setting)
        .add_systems(Startup, setup_mesh_and_animation)
        .add_systems(Update, mouse_motion_system)
        .add_systems(Update, (ray_cast_system, pen_drop));
}

// An example asset that contains a mesh and animation.
const GLTF_PATH: &str = "models/marker_1.glb";

// A component that stores a reference to an animation we want to play. This is
// created when we start loading the mesh (see `setup_mesh_and_animation`) and
// read when the mesh has spawned (see `play_animation_once_loaded`).
#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

fn setup_mesh_and_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Create an animation graph containing a single animation. We want the "run"
    // animation from our example asset, which has an index of two.
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GLTF_PATH)),
    );

    // Store the animation graph as an asset.
    let graph_handle = graphs.add(graph);

    // Create a component that stores a reference to our animation.
    let animation_to_play = AnimationToPlay {
        graph_handle,
        index,
    };

    // Start loading the asset as a scene and store a reference to it in a
    // SceneRoot component. This component will automatically spawn a scene
    // containing our mesh once it has loaded.
    let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)));

    // Spawn an entity with our components, and connect it to an observer that
    // will trigger when the scene is loaded and spawned.
    commands
        .spawn((
            animation_to_play,
            mesh_scene,
            Transform::from_scale(Vec3::splat(0.03)).with_rotation(Quat::from_rotation_z(0.5))
            .with_translation(Vec3::new(0.0, 1.1, 1.0)),
        ))
        .observe(play_animation_when_ready);
}

fn ray_cast_system(mut raycast: MeshRayCast, pen: Single<&Transform, With<AnimationToPlay>>, q: Query<&Character>, mut gizmos: Gizmos){
    let ray = Ray3d::new(pen.translation, -Dir3::Y);
    let hits = raycast.cast_ray(ray, &MeshRayCastSettings::default());
    gizmos.line(ray.origin, ray.origin - (Vec3::Y), Color::from(css::RED));
    for (ent, _ray_mesh_hit) in hits {
        println!("{:?}", ent);
        if let Ok(character) = q.get(*ent) {
            println!("Hit a character! Value = {}", character.0);
        }
        // println!("{:?}", hits);
    }
}

fn play_animation_when_ready(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    // The entity we spawned in `setup_mesh_and_animation` is the trigger's target.
    // Start by finding the AnimationToPlay component we added to that entity.
    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
        // The SceneRoot component will have spawned the scene as a hierarchy
        // of entities parented to our entity. Since the asset contained a skinned
        // mesh and animations, it will also have spawned an animation player
        // component. Search our entity's descendants to find the animation player.
        for child in children.iter_descendants(scene_ready.entity) {
            if let Ok(mut player) = players.get_mut(child) {
                // Tell the animation player to start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                // player.play(animation_to_play.index).repeat();
                player.play(animation_to_play.index);

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
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

fn pen_drop(mouse: Res<ButtonInput<MouseButton>>, mut pen: Single<&mut Transform, With<AnimationToPlay>>) {
    if (mouse.pressed(MouseButton::Left)){
        pen.translation.y = 0.988;
    } else{
        pen.translation.y = 1.1;
    }
}

fn mouse_motion_system(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut marker: Single<&mut Transform, With<AnimationToPlay>>,
) {
    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        // println!("{:?}", delta);
        marker.translation += Vec3 {
            x: -delta.x/200.0,
            y: 0.0,
            z: -delta.y/200.0,
        };
        // println!("{:?}", marker.translation);
    }
}
