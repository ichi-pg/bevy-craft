use crate::block::*;
use crate::collision::*;
use crate::rigid_body::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct SpawnID(u64);

#[derive(Component, Clone, Copy)]
pub struct ItemID;

#[derive(Component, Clone, Copy)]
pub struct Amount;

#[derive(Event)]
pub struct ItemPickedUp {
    pub item_id: ItemID,
    pub amount: Amount,
}

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
            Amount,
            SpawnID(rand::thread_rng().r#gen()),
        ));
    }
    // TODO rand resource
    // TODO texture
}

fn pick_up_item(
    item_query: Query<(Entity, &SpawnID, &ItemID, &Amount)>,
    mut event_reader: EventReader<ItemCollided>,
    mut event_writer: EventWriter<ItemPickedUp>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for (entity, spawn_id, item_id, amount) in &item_query {
            if spawn_id.0 == event.spawn_id.0 {
                commands.entity(entity).despawn();
                event_writer.send(ItemPickedUp {
                    item_id: item_id.clone(),
                    amount: amount.clone(),
                });
            }
        }
    }
    // TODO optimize loop count
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemPickedUp>();
        app.add_systems(Update, (spawn_item, pick_up_item));
    }
}
