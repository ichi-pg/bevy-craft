use crate::gravity::*;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity2(pub Vec2);

pub fn add_velocity(
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

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, add_velocity);
    }
}
