use crate::collision::*;
use crate::gravity::*;
use crate::item_stats::*;
use crate::math::*;
use crate::random::*;
use crate::velocity::*;
use bevy::prelude::*;
use rand::RngCore;

#[derive(Component)]
pub struct MobWalk;

#[derive(Component, Deref, DerefMut)]
pub struct HomePosition(pub Vec2);

#[derive(Component)]
pub struct HomeDistanceSquared(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct PrevPosition(pub Vec2);

#[derive(Component)]
pub struct StackSeconds(pub f32);

const STACK_SECONDS: f32 = 0.1;
const RANDOM_FLIP: u32 = 1000;

fn mob_walk(mut query: Query<(&mut Velocity2, &Direction2, &MoveSpeed), With<MobWalk>>) {
    for (mut velocity, direction, move_speed) in &mut query {
        velocity.x = direction.x * move_speed.0;
    }
}

fn mob_jump(
    mut query: Query<(&mut Velocity2, &BlockCollided, &JumpPower), (With<MobWalk>, With<Grounded>)>,
) {
    for (mut velocity, collided, jump_power) in &mut query {
        if collided.repulsion.y < collided.repulsion.x.abs() {
            velocity.y = jump_power.0;
        }
    }
    // TODO margin to wall
    // TODO too high wall
}

fn move_stack_filp(
    mut query: Query<
        (
            &mut Direction2,
            &Transform,
            &mut PrevPosition,
            &mut StackSeconds,
        ),
        With<MobWalk>,
    >,
    time: Res<Time>,
) {
    for (mut direction, transform, mut prev_position, mut stack_seconds) in &mut query {
        if transform.translation.x == prev_position.x {
            if stack_seconds.0 > STACK_SECONDS {
                direction.x = -direction.x;
                stack_seconds.0 = 0.0;
            } else {
                stack_seconds.0 += time.delta_seconds();
            }
        } else {
            stack_seconds.0 = 0.0;
            prev_position.x = transform.translation.x;
            prev_position.y = transform.translation.y;
        }
    }
}

fn move_random_flip(mut query: Query<&mut Direction2, With<MobWalk>>, mut random: ResMut<Random>) {
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
        With<MobWalk>,
    >,
) {
    for (mut direction, transform, position, distance_squared) in &mut query {
        if (transform.translation.x - position.x).pow2() > distance_squared.0 {
            direction.x = -direction.x;
        }
    }
    // TODO y axis
}

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mob_walk,
                mob_jump,
                move_stack_filp,
                move_random_flip,
                mob_home_area,
            ),
        );
    }
    // TODO find player and chase
    // TODO sometimes stop walk and look around
    // TODO A*
}
