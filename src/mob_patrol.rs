use crate::math::*;
use crate::mob_chase::*;
use crate::mob_stroll::*;
use crate::player::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MobPatrol;

#[derive(Component)]
pub struct FindDistanceSquared(pub f32);

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

pub struct MobPatrolPlugin;

impl Plugin for MobPatrolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mob_patrol);
    }
}
