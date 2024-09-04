use crate::chunk::*;
use crate::math::*;
use crate::random::*;
use crate::velocity::*;
use bevy::prelude::*;
use rand::RngCore;

#[derive(Component)]
pub struct MobStroll(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct HomePosition(pub Vec2);

#[derive(Component)]
pub struct HomeDistanceSquared(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct PrevPosition(pub Vec2);

const STACK_SECONDS: f32 = 0.1;
const RANDOM_FLIP: u32 = 1000;

fn mob_stack_filp(
    mut query: Query<
        (
            &mut Direction2,
            &Transform,
            &mut PrevPosition,
            &mut MobStroll,
        ),
        With<InChunk>,
    >,
    time: Res<Time>,
) {
    for (mut direction, transform, mut prev_position, mut timer) in &mut query {
        if transform.translation.x == prev_position.x {
            if timer.0 > STACK_SECONDS {
                direction.x = -direction.x;
                timer.0 = 0.0;
            } else {
                timer.0 += time.delta_seconds();
            }
        } else {
            timer.0 = 0.0;
            prev_position.x = transform.translation.x;
            prev_position.y = transform.translation.y;
        }
    }
    // TODO find hole
}

fn mob_random_flip(
    mut query: Query<&mut Direction2, (With<InChunk>, With<MobStroll>)>,
    mut random: ResMut<Random>,
) {
    for mut direction in &mut query {
        if random.next_u32() % RANDOM_FLIP == 0 {
            direction.x = -direction.x;
        }
    }
}

fn mob_home_area(
    mut query: Query<
        (
            &mut Direction2,
            &Transform,
            &HomePosition,
            &HomeDistanceSquared,
        ),
        (With<InChunk>, With<MobStroll>),
    >,
) {
    for (mut direction, transform, position, distance) in &mut query {
        if (transform.translation.x - position.x).pow2() > distance.0 {
            direction.x = -direction.x;
        }
    }
}

pub struct MobStrollPlugin;

impl Plugin for MobStrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mob_stack_filp, mob_random_flip, mob_home_area));
    }
    // TODO sometimes stop walk and look around
}
