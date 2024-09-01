use crate::atlas::*;
use crate::collision::*;
use crate::hit_test::*;
use crate::item_attribute::*;
use crate::math::*;
use crate::velocity::*;
use crate::z_sort::*;
use bevy::prelude::*;
use bevy_craft::*;
#[derive(Component)]
pub struct Item;

#[derive(Component, Clone, Copy)]
pub struct ItemID(pub u16);

#[derive(Component, Clone, Copy)]
pub struct ItemAmount(pub u16);

#[derive(Event)]
pub struct ItemDropped {
    pub position: Vec2,
    pub item_id: u16,
    pub amount: u16,
}

pub trait ItemAndAmount {
    fn item_id(&self) -> u16;
    fn amount(&self) -> u16;
    fn new(item_id: u16, amount: u16) -> Self;
}

#[derive(Event, ItemAndAmount)]
pub struct ItemPickedUp {
    pub item_id: u16,
    pub amount: u16,
}

fn spawn_item(
    mut event_reader: EventReader<ItemDropped>,
    mut commands: Commands,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
) {
    for event in event_reader.read() {
        let Some(attribute) = attribute_map.get(&event.item_id) else {
            continue;
        };
        let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
            continue;
        };
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                texture: atlas.texture.clone(),
                transform: Transform::from_translation(event.position.with_z(ITEM_Z)),
                ..default()
            },
            TextureAtlas {
                layout: atlas.layout.clone(),
                index: attribute.atlas_index as usize,
            },
            Shape::Circle(32.0),
            Velocity2::default(),
            Item,
            ItemID(event.item_id),
            ItemAmount(event.amount),
        ));
    }
}

fn pick_up_item(
    query: Query<(Entity, &ItemID, &ItemAmount), With<ItemCollided>>,
    mut event_writer: EventWriter<ItemPickedUp>,
    mut commands: Commands,
) {
    for (entity, item_id, amount) in &query {
        commands.entity(entity).despawn_recursive();
        event_writer.send(ItemPickedUp {
            item_id: item_id.0,
            amount: amount.0,
        });
    }
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemDropped>();
        app.add_event::<ItemPickedUp>();
        app.add_systems(Update, spawn_item);
        app.add_systems(Last, pick_up_item);
    }
}
