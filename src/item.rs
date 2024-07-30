use crate::block::*;
use crate::collision::*;
use crate::rigid_body::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct SpawnID(u64);

#[derive(Component)]
pub struct ItemID;

#[derive(Component)]
struct ItemAmount;

fn spawn_item(mut event_reader: EventReader<BlockDestroied>, mut commands: Commands) {
    for event in event_reader.read() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.6, 0.6, 0.6),
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                transform: event.transform.clone(),
                ..default()
            },
            Collider::circle(32.0),
            BroadBlocks::default(),
            NarrowBlocks::default(),
            Velocity2::default(),
            ItemID,
            ItemAmount,
            SpawnID(rand::thread_rng().r#gen()),
        ));
    }
    // TODO rand resource
}

fn pick_up_item(
    query: Query<(Entity, &SpawnID), With<ItemID>>,
    mut event_reader: EventReader<ItemCollided>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for (entity, spawn_id) in &query {
            if spawn_id.0 == event.spawn_id.0 {
                commands.entity(entity).despawn();
            }
        }
    }
    // TODO into inventory
    // TODO optimize loop count
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_item, pick_up_item));
    }
}
