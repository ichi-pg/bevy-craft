use crate::chunk::*;
use crate::gravity::*;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity2(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct Direction2(pub Vec2);

#[derive(Component)]
pub struct KnockBack;

pub fn add_velocity(
    mut query: Query<(&mut Transform, &Velocity2), With<InChunk>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        if velocity.0 == Vec2::ZERO {
            continue;
        }
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn remove_knock_back(
    mut query: Query<Entity, (With<KnockBack>, With<Grounded>)>,
    mut commands: Commands,
) {
    for entity in &mut query {
        commands.entity(entity).remove::<KnockBack>();
    }
    // TODO grounded or damage interval
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, remove_knock_back);
        app.add_systems(PostUpdate, add_velocity);
    }
}
