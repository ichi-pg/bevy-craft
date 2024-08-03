use crate::block::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Grounded;

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

pub struct GroundedPlugin;

impl Plugin for GroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, remove_grounded);
    }
}
