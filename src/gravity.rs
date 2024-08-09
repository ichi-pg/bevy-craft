use crate::block::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct JumpController;

#[derive(Component)]
pub struct Grounded;

fn add_gravity(mut query: Query<&mut Velocity2, Without<Grounded>>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.y = (velocity.y - 4000.0 * time.delta_seconds()).max(-2048.0);
    }
}

fn stop_gravity(mut query: Query<&mut Velocity2, (Without<JumpController>, With<Grounded>)>) {
    for mut velocity in &mut query {
        velocity.y = 0.0;
    }
}

fn remove_grounded(
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

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_gravity, stop_gravity, remove_grounded));
    }
}
