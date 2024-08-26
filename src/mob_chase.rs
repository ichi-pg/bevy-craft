use crate::gravity::*;
use crate::math::*;
use crate::mob_jump_attack::*;
use crate::mob_patrol::*;
use crate::mob_stroll::*;
use crate::mob_walk::*;
use crate::player::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MobChase;

#[derive(Component)]
pub struct LostDistanceSquared(pub f32);

#[derive(Component)]
pub struct AttackDistanceSquared(pub f32);

fn mob_chase(
    mut query: Query<(&mut Direction2, &Transform), (With<MobChase>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (mut direction, transform) in &mut query {
        for player_transform in &player_query {
            direction.x = transform
                .translation
                .x
                .look_at(player_transform.translation.x);
        }
    }
}

fn mob_chase_lost(
    mut query: Query<
        (Entity, &Transform, &LostDistanceSquared, &mut HomePosition),
        (With<MobChase>, Without<Player>, With<Grounded>),
    >,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    for (entity, transform, distance, mut home_position) in &mut query {
        for player_transform in &player_query {
            if (player_transform.translation.x - transform.translation.x).pow2() > distance.0 {
                commands
                    .entity(entity)
                    .remove::<MobChase>()
                    .insert(MobPatrol)
                    .insert(MobStroll(0.0));
                home_position.0 = transform.translation.xy();
            }
        }
    }
}

fn mob_chase_attack(
    query: Query<
        (Entity, &Transform, &AttackDistanceSquared),
        (With<MobChase>, Without<Player>, With<Grounded>),
    >,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    for (entity, transform, distance) in &query {
        for player_transform in &player_query {
            if (player_transform.translation.x - transform.translation.x).pow2() < distance.0 {
                commands
                    .entity(entity)
                    .remove::<MobChase>()
                    .remove::<MobWalk>()
                    .insert(MobJumpAttack(0.0));
            }
        }
    }
}

pub struct MobChasePlugin;

impl Plugin for MobChasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (mob_chase, mob_chase, mob_chase_lost, mob_chase_attack),
        );
    }
    // FIXME enemy don't move at end of find range
    // TODO which player?
    // TODO y axis
    // TODO filter components with states
    // TODO flying without grounded
    // TODO debug states text
}
