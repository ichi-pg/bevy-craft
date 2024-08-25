use crate::gravity::*;
use crate::item_stats::*;
use crate::math::*;
use crate::mob_chase::*;
use crate::mob_walk::*;
use crate::player::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MobJumpAttack;

#[derive(Component)]
pub struct MobJumpAttacked;

#[derive(Component)]
pub struct AttackTimer(pub f32);

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
        for player_transform in &player_query {
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

pub struct MobJumpAttackPlugin;

impl Plugin for MobJumpAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mob_jump_attack, mob_jump_attacked));
    }
    // TODO melee or ranged
    // TODO attack pattern value components to constants?
}
