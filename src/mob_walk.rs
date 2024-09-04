use crate::chunk::*;
use crate::collision::*;
use crate::gravity::*;
use crate::stats::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MobWalk;

fn mob_walk(
    mut query: Query<
        (&mut Velocity2, &Direction2, &MoveSpeed),
        (With<InChunk>, With<MobWalk>, Without<KnockBack>),
    >,
) {
    for (mut velocity, direction, move_speed) in &mut query {
        velocity.x = direction.x * move_speed.0;
    }
}

fn mob_jump(
    mut query: Query<
        (&mut Velocity2, &BlockCollided, &JumpPower),
        (
            With<InChunk>,
            With<MobWalk>,
            With<Grounded>,
            Without<KnockBack>,
        ),
    >,
) {
    for (mut velocity, collided, jump_power) in &mut query {
        if collided.repulsion.y < collided.repulsion.x.abs() {
            velocity.y = jump_power.0;
        }
    }
    // TODO margin to wall
    // TODO too high wall
}

pub struct MobWalkPlugin;

impl Plugin for MobWalkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mob_walk, mob_jump));
    }
    // TODO A*?
}
