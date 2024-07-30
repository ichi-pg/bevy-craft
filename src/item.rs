use crate::block::*;
use crate::collision::*;
use crate::rigid_body::*;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct ItemID(i32);

#[derive(Component, Deref, DerefMut, Default)]
struct ItemAmount(i32);

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
            BroadHits::default(),
            NarrowHits::default(),
            Velocity2::default(),
            ItemID(1),
            ItemAmount(1),
        ));
    }
}

fn pick_up_item(
    query: Query<(Entity, &Transform), With<ItemID>>,
    mut event_reader: EventReader<Collided>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for (entity, transform) in &query {
            if transform.translation.xy() == event.pos {
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
