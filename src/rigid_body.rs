use crate::collision::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct RigitBodyController;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity2(Vec2);

fn add_velocity(
    mut players: Query<(Entity, &mut Transform, &Velocity2)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, velocity) in &mut players {
        if velocity.0 == Vec2::ZERO {
            continue;
        }
        if velocity.y >= 0.0 {
            commands.entity(entity).remove::<Grounded>();
        }
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
    // TODO remove grounded when destroied bottom block
}

fn add_gravity(mut players: Query<&mut Velocity2, Without<Grounded>>, time: Res<Time>) {
    for mut velocity in &mut players {
        velocity.y = (velocity.y - 4000.0 * time.delta_seconds()).max(-2048.0);
    }
}

fn stop_gravity(
    mut players: Query<&mut Velocity2, (Without<RigitBodyController>, With<Grounded>)>,
) {
    for mut velocity in &mut players {
        velocity.y = 0.0;
    }
}

pub struct RigitBodyPlugin;

impl Plugin for RigitBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_gravity, add_velocity, stop_gravity));
    }
}
