use crate::block::*;
use crate::collision::*;
use crate::hit_test::*;
use crate::rigid_body::*;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct ItemID(pub u16);

#[derive(Component, Clone, Copy)]
pub struct Amount(pub u16);

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
                transform: event.transform,
                ..default()
            },
            Shape::Circle(32.0),
            Velocity2::default(),
            ItemID(1),
            Amount(1),
        ));
    }
    // TODO texture
}

fn pick_up_item(
    query: Query<(Entity, &ItemID, &Amount), With<Collided>>,
    mut event_writer: EventWriter<ItemPickedUp>,
    mut commands: Commands,
) {
    for (entity, item_id, amount) in &query {
        commands.entity(entity).despawn();
        event_writer.send(ItemPickedUp {
            item_id: *item_id,
            amount: *amount,
        });
    }
}

fn sync_amount(mut query: Query<(&Amount, &mut Text), Changed<Amount>>) {
    for (amount, mut text) in &mut query {
        for section in &mut text.sections {
            section.value = format!("{}", amount.0);
        }
    }
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemPickedUp>();
        app.add_systems(Update, (spawn_item, sync_amount));
        app.add_systems(PostUpdate, pick_up_item);
    }
}
