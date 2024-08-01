use crate::grounded::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct RigitBodyController;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity2(pub Vec2);

fn add_velocity(
    mut query: Query<(Entity, &mut Transform, &Velocity2)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, velocity) in &mut query {
        if velocity.0 == Vec2::ZERO {
            continue;
        }
        if velocity.y >= 0.0 {
            commands.entity(entity).remove::<Grounded>();
        }
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn add_gravity(mut query: Query<&mut Velocity2, Without<Grounded>>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.y = (velocity.y - 4000.0 * time.delta_seconds()).max(-2048.0);
    }
}

fn stop_gravity(mut query: Query<&mut Velocity2, (Without<RigitBodyController>, With<Grounded>)>) {
    for mut velocity in &mut query {
        velocity.y = 0.0;
    }
}

pub struct RigitBodyPlugin;

impl Plugin for RigitBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_gravity, add_velocity, stop_gravity));
    }
}
