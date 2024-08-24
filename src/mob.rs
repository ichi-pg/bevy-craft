use crate::collision::*;
use crate::item_stats::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MobWalk;

#[derive(Component)]
pub struct HomePosition(pub Vec2);

#[derive(Component)]
pub struct HomeDistanceSquared(pub f32);

fn mob_walk(mut query: Query<(&mut Velocity2, &Direction2, &MoveSpeed), With<MobWalk>>) {
    for (mut velocity, direction, move_speed) in &mut query {
        velocity.x = direction.x * move_speed.0;
    }
    // TODO find player and chase
    // TODO stay home area
    // TODO sometimes stop walk and look around
}

fn mob_collided(mut query: Query<(&mut Velocity2, &BlockCollided, &JumpPower), With<MobWalk>>) {
    for (mut velocity, collided, jump_power) in &mut query {
        if collided.repulsion.y < collided.repulsion.x.abs() {
            velocity.y = jump_power.0;
        }
    }
    // FIXME stop velocity.x in second jumping
    // TODO margin to wall
    // TODO too high wall
    // TODO with grounded
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
    for (mut direction, transform, position, distance) in &mut query {
        if transform.translation.xy().distance_squared(position.0) > distance.0 {
            direction.x = -direction.x;
        }
    }
}

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mob_walk, mob_collided, mob_home_area));
    }
}