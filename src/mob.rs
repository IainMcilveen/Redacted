use std::f32::consts::PI;

use bevy::math::ops::{floor, sin};
use bevy::prelude::*;
use bevy_sprite3d::Sprite3d;
use rand::Rng;

use crate::environment::{GlassCrackStage, PIXELS_PER_METRE};
use crate::loading::GameAssets;
use crate::{CountdownTimer, GameState, LIFETIME};

pub const MAX_MOB_MEMBERS: u32 = 32;
pub const MOB_ATTACK_ADVANCE: f32 = 10.0;
pub const GLASS_BREAK_STAGE: usize = 9;

#[derive(Component)]
struct MobMember {
    anger: f32,
    offset: f32,
    z: f32,
}

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(GameState::PLAYING), setup)
    app.add_systems(Update, update_mob.run_if(in_state(GameState::PLAYING)));
}

// fn setup(mut commands: Commands, assets: Res<GameAssets>) {}

fn update_mob(
    countdown: Res<CountdownTimer>,
    mut members: Query<(&mut Transform, &MobMember)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    glass_crack_stage: ResMut<GlassCrackStage>,
) {
    let progress = 1.0 - countdown.0.remaining().as_secs_f32() / LIFETIME;
    let target_number_of_mob_members = floor(progress * MAX_MOB_MEMBERS as f32) as usize;
    let mob_members = members.count();
    if mob_members < target_number_of_mob_members {
        spawn_mob(&mut commands, &assets);
    }
    let mob_attack_duration = LIFETIME * (1.0 - (GLASS_BREAK_STAGE as f32) / 11.0);
    let mob_attack_progress = 1.0 - (countdown.0.remaining_secs() / mob_attack_duration);
    println!(
        "glass_crack_stage: {} mob_attack_progress: {}",
        glass_crack_stage.0, mob_attack_progress
    );
    for mut member in &mut members {
        let sway = sin(countdown.0.elapsed_secs() * 8.0 + member.1.offset) * 30.0;
        member.0.rotation = Quat::from_rotation_z(PI * 2.0 * sway * member.1.anger / 360.0);
        if glass_crack_stage.0 >= GLASS_BREAK_STAGE {
            member.0.translation.z = member.1.z - (MOB_ATTACK_ADVANCE * member.1.anger) * mob_attack_progress;
        }
    }
}

fn spawn_mob(commands: &mut Commands, assets: &Res<GameAssets>) {
    let mut rng = rand::rng();
    let z = 11.0 + rng.random_range(0.0..4.0);
    commands.spawn((
        MobMember {
            anger: rng.random_range(0.0..1.0),
            offset: rng.random_range(0.0..PI * 2.0),
            z,
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
        Transform::from_xyz(rng.random_range(-10.0..10.0), 0.5, z),
        DespawnOnExit(GameState::PLAYING),
    ));
}
