use crate::block::*;
use crate::collision::*;
use crate::layer::*;
use crate::rigid_body::*;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
struct ItemID(pub i32);

#[derive(Component, Deref, DerefMut, Default)]
struct ItemAmount(pub i32);

fn spawn_item(mut events: EventReader<BlockDestroied>, mut commands: Commands) {
    for event in events.read() {
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
            Collider::circle(32.0, ITEM, BLOCK),
            BroadHits::default(),
            NarrowHits::default(),
            Velocity2::default(),
            ItemID(1),
            ItemAmount(1),
        ));
    }
    // TODO pick up
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_item);
    }
}
