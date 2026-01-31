use bevy::camera::RenderTarget;
use bevy::{camera::visibility::RenderLayers, prelude::*, render::render_resource::TextureFormat};

use crate::paper::{CAMERA_POS, PAPER_POS};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, mouse_draw_system);
}

#[derive(Component)]
struct SecondaryCamera;

const CANVAS_LAYER: RenderLayers = RenderLayers::layer(1);

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
            clear_color: Color::WHITE.into(),
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
            unlit: true, // Makes the "drawing" easier to see
            ..default()
        })),
        Transform::from_translation(PAPER_POS),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(CAMERA_POS).looking_at(PAPER_POS, Vec3::Y),
    ));
}

fn mouse_draw_system(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), (With<Camera3d>, Without<SecondaryCamera>)>,
    mut commands: Commands,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let window = window.single().unwrap();
    let (camera, camera_transform) = camera_q.single().unwrap();

    // Convert mouse screen position to a 3D ray
    if let Some(ray) = window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world(camera_transform, pos).ok())
    {
        // Intersect ray with the ground plane (Normal: Up, Point: Zero)
        if let Some(distance) = ray.intersect_plane(PAPER_POS, InfinitePlane3d::new(Vec3::Y)) {
            let hit_point = ray.get_point(distance);

            // Normalize the hit point relative to the paper planes' center
            // hit_point.x is already relative to 0.0
            // hit_point.z needs to be relative to 1.0 (the PAPER_POS.z)
            let local_x = hit_point.x;
            let local_z = hit_point.z - PAPER_POS.z;

            // Map to Texture Coordinates (0.6 world units = 600px -> Scale 1000)
            // We negate local_z so that moving the mouse "forward" (+Z) maps correctly to the 2D canvas
            let canvas_x = local_x * 1000.0;
            let canvas_y = -local_z * 1000.0;

            println!("{:?} {:?}", canvas_x, canvas_y);

            // Spawn a "brush stroke" sprite on the hidden canvas layer
            // Map the 3D X/Z to 2D X/Y for the canvas camera
            commands.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(50.0)),
                Transform::from_xyz(canvas_x, canvas_y, 0.0),
                CANVAS_LAYER,
            ));
        }
    }
}
