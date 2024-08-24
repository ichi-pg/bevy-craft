use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity2(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct Direction2(pub Vec2);

pub fn add_velocity(mut query: Query<(&mut Transform, &Velocity2)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        if velocity.0 == Vec2::ZERO {
            continue;
        }
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, add_velocity);
    }
}
