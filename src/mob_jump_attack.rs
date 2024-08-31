use crate::gravity::*;
use crate::math::*;
use crate::mob_chase::*;
use crate::mob_walk::*;
use crate::player::*;
use crate::stats::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MobJumpAttack(pub f32);

#[derive(Component)]
pub struct MobJumpAttacked(pub f32);

const CHARGE_POWER: f32 = 1.2;
const MAX_CHARGE_POWER: f32 = 512.0;
const ATTACK_DELAY: f32 = 1.0;
const COOL_DOWN: f32 = 1.0;

fn mob_jump_attack(
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut Direction2,
            &mut Velocity2,
            &mut MobJumpAttack,
            &AttackSpeed,
            &JumpPower,
        ),
        (Without<Player>, Without<KnockBack>),
    >,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform, mut direction, mut velocity, mut timer, attack_speed, jump_power) in
        &mut query
    {
        for player_transform in &player_query {
            if timer.0 > ATTACK_DELAY {
                velocity.x = direction.x
                    * ((player_transform.translation.x - transform.translation.x) * CHARGE_POWER)
                        .abs()
                        .min(MAX_CHARGE_POWER);
                velocity.y = jump_power.0;
                commands
                    .entity(entity)
                    .remove::<MobJumpAttack>()
                    .insert(MobJumpAttacked(0.0));
            } else {
                direction.x = transform
                    .translation
                    .x
                    .look_at(player_transform.translation.x);
                velocity.0 = Vec2::ZERO;
                timer.0 += time.delta_seconds() * attack_speed.0;
            }
        }
    }
    // TODO calculate charge power with player position
}

fn mob_jump_attacked(
    mut query: Query<
        (Entity, &mut Velocity2, &mut MobJumpAttacked, &AttackSpeed),
        (With<Grounded>, Without<KnockBack>),
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut velocity, mut timer, attack_speed) in &mut query {
        if timer.0 > COOL_DOWN {
            commands
                .entity(entity)
                .remove::<MobJumpAttacked>()
                .insert(MobChase)
                .insert(MobWalk);
        } else {
            velocity.0 = Vec2::ZERO;
            timer.0 += time.delta_seconds() * attack_speed.0;
        }
    }
}

pub struct MobJumpAttackPlugin;

impl Plugin for MobJumpAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mob_jump_attack, mob_jump_attacked));
    }
    // TODO projectile
    // TODO melee attack
    // TODO flying attack
    // TODO charge attack
}
