use crate::collision::*;
use crate::item_stats::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MobWalk;

#[derive(Component)]
pub struct HomePosition(pub Vec2);

#[derive(Component)]
pub struct HomeDistance(pub f32);

fn mob_walk(mut query: Query<(&mut Velocity2, &Direction2, &MoveSpeed), With<MobWalk>>) {
    for (mut velocity, direction, move_speed) in &mut query {
        velocity.x = direction.x * move_speed.0;
    }
    // TODO find player and chase
    // TODO stay home area
    // TODO sometimes stop walk and look around
}

fn mob_collided(mut query: Query<(&mut Direction2, &Collided), With<MobWalk>>) {
    for (mut direction, collided) in &mut query {
        if collided.y < collided.x.abs() {
            direction.x = -direction.x;
        }
    }
    // TODO jump
}

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mob_walk, mob_collided));
    }
}
