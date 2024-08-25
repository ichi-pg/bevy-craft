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

#[derive(Component)]
pub struct MobJumpAttack;

#[derive(Component)]
pub struct MobJumpAttacked;

#[derive(Component, Deref, DerefMut)]
pub struct HomePosition(pub Vec2);

#[derive(Component)]
pub struct HomeDistanceSquared(pub f32);

#[derive(Component)]
pub struct FindDistanceSquared(pub f32);

#[derive(Component)]
pub struct LostDistanceSquared(pub f32);

#[derive(Component)]
pub struct AttackDistanceSquared(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct PrevPosition(pub Vec2);

#[derive(Component)]
pub struct StackTimer(pub f32);

#[derive(Component)]
pub struct AttackTimer(pub f32);

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
            &mut StackTimer,
        ),
        With<MobStroll>,
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
}

fn mob_patrol(
    query: Query<(Entity, &Transform, &FindDistanceSquared), (With<MobPatrol>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
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
}

fn mob_chase(
    mut query: Query<(&mut Direction2, &Transform), (With<MobChase>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player_transform in &player_query {
        for (mut direction, transform) in &mut query {
            direction.x = transform
                .translation
                .x
                .look_at(player_transform.translation.x);
        }
    }
}

fn mob_chase_lost(
    query: Query<(Entity, &Transform, &LostDistanceSquared), (With<MobChase>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    for player_transform in &player_query {
        for (entity, transform, distance) in &query {
            if (player_transform.translation.x - transform.translation.x).pow2() > distance.0 {
                commands.entity(entity).remove::<MobChase>();
                commands.entity(entity).insert(MobPatrol);
                commands.entity(entity).insert(MobStroll);
            }
        }
    }
}

fn mob_chase_attack(
    mut query: Query<
        (Entity, &Transform, &AttackDistanceSquared, &mut AttackTimer),
        (With<MobChase>, Without<Player>, With<Grounded>),
    >,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    for player_transform in &player_query {
        for (entity, transform, distance, mut timer) in &mut query {
            if (player_transform.translation.x - transform.translation.x).pow2() < distance.0 {
                commands.entity(entity).remove::<MobChase>();
                commands.entity(entity).remove::<MobWalk>();
                commands.entity(entity).insert(MobJumpAttack);
                timer.0 = 0.0;
            }
        }
    }
}

fn mob_jump_attack(
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut Direction2,
            &mut Velocity2,
            &mut AttackTimer,
            &AttackSpeed,
            &AttackDelay,
            &JumpPower,
        ),
        (With<MobJumpAttack>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for player_transform in &player_query {
        for (
            entity,
            transform,
            mut direction,
            mut velocity,
            mut timer,
            attack_speed,
            attack_delay,
            jump_power,
        ) in &mut query
        {
            if timer.0 > attack_delay.0 {
                velocity.x = direction.x * attack_speed.0;
                velocity.y = jump_power.0;
                timer.0 = 0.0;
                commands.entity(entity).remove::<MobJumpAttack>();
                commands.entity(entity).insert(MobJumpAttacked);
            } else {
                direction.x = transform
                    .translation
                    .x
                    .look_at(player_transform.translation.x);
                velocity.0 = Vec2::ZERO;
                timer.0 += time.delta_seconds();
            }
        }
    }
}

fn mob_jump_attacked(
    mut query: Query<
        (Entity, &mut Velocity2, &mut AttackTimer, &AttackCoolDown),
        (With<MobJumpAttacked>, With<Grounded>),
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut velocity, mut timer, cool_down) in &mut query {
        if timer.0 > cool_down.0 {
            commands.entity(entity).remove::<MobJumpAttacked>();
            commands.entity(entity).insert(MobChase);
            commands.entity(entity).insert(MobWalk);
        } else {
            velocity.0 = Vec2::ZERO;
            timer.0 += time.delta_seconds();
        }
    }
    // TODO moving cool time?
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
                mob_chase_lost,
                mob_chase_attack,
                mob_jump_attack,
                mob_jump_attacked,
            ),
        );
    }
    // TODO sometimes stop walk and look around
    // TODO A*?
    // TODO which player?
    // TODO y axis
    // TODO melee or ranged
    // TODO filter components with states
    // TODO attack pattern value components to constants?
}
