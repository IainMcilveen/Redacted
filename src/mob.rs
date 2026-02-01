use std::f32::consts::PI;

use bevy::math::ops::{floor, sin};
use bevy::prelude::*;
use bevy_sprite3d::Sprite3d;
use rand::Rng;

use crate::environment::PIXELS_PER_METRE;
use crate::loading::GameAssets;
use crate::{GameState, LIFETIME};

pub const MAX_MOB_MEMBERS: u32 = 32;

#[derive(Component)]
struct MobMember {
    anger: f32,
    offset: f32,
}

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(GameState::PLAYING), setup)
    app.add_systems(Update, update_mob.run_if(in_state(GameState::PLAYING)));
}

// fn setup(mut commands: Commands, assets: Res<GameAssets>) {}

fn update_mob(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MobMember)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let progress = time.elapsed_secs() / LIFETIME;
    let target_number_of_mob_members = floor(progress * MAX_MOB_MEMBERS as f32) as usize;
    let mob_members = query.count();
    if mob_members < target_number_of_mob_members {
        spawn_mob(&mut commands, &assets);
    }
    for mut member in &mut query {
        let sway = sin(time.elapsed_secs() * 8.0 + member.1.offset) * 30.0;
        member.0.rotation = Quat::from_rotation_z(PI * 2.0 * sway * member.1.anger / 360.0);
    }
}

fn spawn_mob(commands: &mut Commands, assets: &Res<GameAssets>) {
    let mut rng = rand::rng();
    commands.spawn((
        MobMember {
            anger: rng.random_range(0.0..1.0),
            offset: rng.random_range(0.0..PI * 2.0),
        },
        Sprite::from_image(
            assets.mob_sprites[rng.random_range(0..assets.mob_sprites.len())].clone(),
        ),
        Sprite3d {
            pixels_per_metre: PIXELS_PER_METRE,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        },
        Transform::from_xyz(
            rng.random_range(-10.0..10.0),
            0.5,
            11.0 + rng.random_range(0.0..4.0),
        ),
        DespawnOnExit(GameState::PLAYING),
    ));
}
