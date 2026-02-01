use bevy::camera::RenderTarget;
use bevy::{camera::visibility::RenderLayers, prelude::*, render::render_resource::TextureFormat};

use crate::audio::{SoundEvent, Sounds};
use crate::paper::PAPER_POS;
use crate::pen::{InkSupplyPercent, Marker};

#[derive(Resource, Default)]
struct BrushState {
    last_pos: Option<Vec2>,
}

#[derive(Component)]
struct SecondaryCamera;

#[derive(Component)]
pub struct PaintPlane;

const CANVAS_LAYER: RenderLayers = RenderLayers::layer(1);

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BrushState>()
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_draw_system);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // This is the texture that will be rendered to.
    let image = Image::new_target_texture(
        600,
        1000,
        TextureFormat::Rgba8Unorm,
        Some(TextureFormat::Rgba8UnormSrgb),
    );

    let image_handle = images.add(image);

    // Secondary Camera
    commands.spawn((
        Camera2d::default(),
        Camera {
            // render before the "main pass" camera
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        RenderTarget::Image(image_handle.clone().into()),
        Transform::IDENTITY,
        CANVAS_LAYER,
        SecondaryCamera,
    ));

    // plane to draw onto
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(0.6, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(image_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true, // Makes the "drawing" easier to see
            ..default()
        })),
        Transform::from_translation(PAPER_POS + Vec3::Y * 0.002),
        Pickable::IGNORE,
        PaintPlane,
    ));
}

fn mouse_draw_system(
    buttons: Res<ButtonInput<MouseButton>>,
    marker_q: Single<(&Marker, &mut InkSupplyPercent), With<Marker>>,
    mut brush_state: ResMut<BrushState>,
    mut commands: Commands,
) {
    if !buttons.pressed(MouseButton::Left) {
        brush_state.last_pos = None;
        return;
    }

    let (marker, mut ink_supply) = marker_q.into_inner();
    let location: Vec3;
    if let Some(val) = marker.tip_location {
        location = val;
    } else {
        return;
    }

    let local_x = location.x;
    let local_z = location.z - PAPER_POS.z;

    // Map to Texture Coordinates (0.6 world units = 600px -> Scale 1000)
    // We negate local_z so that moving the mouse "forward" (+Z) maps correctly to the 2D canvas
    let canvas_x = local_x * 1000.0;
    let canvas_y = -local_z * 1000.0;
    let current_pos = Vec2::new(canvas_x, canvas_y);

    // If we have a previous point, interpolate
    if let Some(last_pos) = brush_state.last_pos {
        let dist = last_pos.distance(current_pos);
        let step_size = 2.0; // Lower = smoother line, higher = better performance
        let steps = (dist / step_size).ceil() as i32;

        for i in 0..steps {
            let lerped_pos = last_pos.lerp(current_pos, i as f32 / steps as f32);

            commands.spawn((
                Sprite::from_color(Color::srgb(0.0, 0.0, 0.0), Vec2::splat(25.0)),
                Transform::from_xyz(lerped_pos.x, lerped_pos.y, 0.0),
                CANVAS_LAYER,
            ));
        }
        
        if !ink_supply.1{
            println!("{:?}", ink_supply.0);
            let distance = last_pos.distance(current_pos);
            // if ink_supply
            ink_supply.0 -= distance / 10.0;
        }
        // println!("{}", ink_supply.0);
    } else {
        // commands.trigger(SoundEvent {
        //     sound: Sounds::VineBoom,
        //     setting: PlaybackSettings::DESPAWN,
        // });
        // First click stroke
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(15.0)),
            Transform::from_xyz(current_pos.x, current_pos.y, 0.0),
            CANVAS_LAYER,
        ));
    }

    brush_state.last_pos = Some(current_pos);
}
