use crate::collision::*;
use crate::gravity::*;
use crate::item_stats::*;
use crate::math::*;
use crate::player::*;
use crate::random::*;
use crate::velocity::*;
use bevy::prelude::*;
use rand::RngCore;

#[derive(Component)]
pub struct MobWalk;

#[derive(Component)]
pub struct MobPatrol;

#[derive(Component)]
pub struct MobStroll;

#[derive(Component)]
pub struct MobChase;

#[derive(Component, Deref, DerefMut)]
pub struct HomePosition(pub Vec2);

#[derive(Component)]
pub struct HomeDistanceSquared(pub f32);

#[derive(Component)]
pub struct PatrolDistanceSquared(pub f32);

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

fn mob_stack_filp(
    mut query: Query<
        (
            &mut Direction2,
            &Transform,
            &mut PrevPosition,
            &mut StackSeconds,
        ),
        With<MobStroll>,
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

fn mob_random_flip(mut query: Query<&mut Direction2, With<MobStroll>>, mut random: ResMut<Random>) {
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
        With<MobStroll>,
    >,
) {
    for (mut direction, transform, position, distance) in &mut query {
        if (transform.translation.x - position.x).pow2() > distance.0 {
            direction.x = -direction.x;
        }
    }
    // TODO y axis
}

fn mob_patrol(
    query: Query<(Entity, &Transform, &PatrolDistanceSquared), With<MobPatrol>>,
    player_query: Query<&Transform, (With<Player>, Without<MobPatrol>)>,
    mut commands: Commands,
) {
    for player_transform in &player_query {
        for (entity, transform, distance) in &query {
            if (player_transform.translation.x - transform.translation.x).pow2() < distance.0 {
                commands.entity(entity).remove::<MobPatrol>();
                commands.entity(entity).remove::<MobStroll>();
                commands.entity(entity).insert(MobChase);
            }
        }
    }
    // TODO chunk or sweep or tree
    // TODO y axis
}

fn mob_chase(
    mut query: Query<(Entity, &mut Direction2, &Transform, &PatrolDistanceSquared), With<MobChase>>,
    player_query: Query<&Transform, (With<Player>, Without<MobChase>)>,
    mut commands: Commands,
) {
    for player_transform in &player_query {
        for (entity, mut direction, transform, distance) in &mut query {
            if (player_transform.translation.x - transform.translation.x).pow2() > distance.0 {
                commands.entity(entity).insert(MobPatrol);
                commands.entity(entity).insert(MobStroll);
                commands.entity(entity).remove::<MobChase>();
            } else {
                direction.x = if transform.translation.x < player_transform.translation.x {
                    1.0
                } else {
                    -1.0
                }
            }
        }
    }
    // TODO which player?
    // TODO split find and lost player distance
    // TODO stop and attack
    // TODO y axis
}

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mob_walk,
                mob_chase,
                mob_jump,
                mob_stack_filp,
                mob_random_flip,
                mob_home_area,
                mob_patrol,
                mob_chase,
            ),
        );
    }
    // TODO sometimes stop walk and look around
    // TODO A*
}
