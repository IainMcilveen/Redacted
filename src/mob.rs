use std::f32::consts::PI;

use bevy::math::ops::sin;
use bevy::prelude::*;
use bevy_sprite3d::Sprite3d;
use rand::Rng;

use crate::GameState;
use crate::environment::PIXELS_PER_METRE;
use crate::loading::GameAssets;

#[derive(Component)]
struct MobMember {
    anger: f32,
    offset: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::PAGETEST), setup)
        .add_systems(Update, update_mob);
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    let mut rng = rand::rng();
    let mob_size = 32;
    for i in 0..mob_size {
        commands.spawn((
            MobMember {
                anger: rng.random_range(0.0..1.0),
                offset: rng.random_range(0.0..PI * 2.0),
            },
            Sprite::from_image(assets.mob_sprites[i % assets.mob_sprites.len()].clone()),
            Sprite3d {
                pixels_per_metre: PIXELS_PER_METRE,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            },
            Transform::from_xyz(rng.random_range(-10.0..10.0), 0.5, 11.0 + rng.random_range(0.0..4.0)),
        ));
    }
}

fn update_mob(time: Res<Time>, mut query: Query<(&mut Transform, &MobMember)>) {
    for mut member in &mut query {
        let sway = sin(time.elapsed_secs() * 8.0 + member.1.offset) * 30.0;
        member.0.rotation = Quat::from_rotation_z(PI * 2.0 * sway * member.1.anger / 360.0);
    }
}
