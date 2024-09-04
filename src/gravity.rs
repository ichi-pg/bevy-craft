use crate::block::*;
use crate::chunk::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Grounded;

fn add_gravity(
    mut query: Query<&mut Velocity2, (Without<Grounded>, With<InChunk>)>,
    time: Res<Time>,
) {
    for mut velocity in &mut query {
        velocity.y = (velocity.y - 4000.0 * time.delta_seconds()).max(-2048.0);
    }
}

fn block_destroied(
    query: Query<Entity, With<Grounded>>,
    mut commands: Commands,
    event_reader: EventReader<BlockDestroied>,
) {
    if event_reader.is_empty() {
        return;
    }
    for entity in &query {
        commands.entity(entity).remove::<Grounded>();
    }
}

pub fn remove_grounded(mut query: Query<(Entity, &Velocity2)>, mut commands: Commands) {
    for (entity, velocity) in &mut query {
        if velocity.0 == Vec2::ZERO {
            continue;
        }
        commands.entity(entity).remove::<Grounded>();
    }
}

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                add_gravity,
                (remove_grounded, block_destroied).before(add_gravity),
            )
                .before(add_velocity),
        );
    }
}
