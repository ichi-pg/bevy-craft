use crate::atlas::*;
use crate::block::*;
use crate::camera::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::item_node::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component)]
struct UnloadItem;

#[derive(Component, Default)]
pub struct Storage;

#[derive(Component, Default)]
pub struct StorageItem;

#[derive(Resource)]
struct StorageBlockID(u64);

#[derive(Event)]
pub struct StorageClicked {
    pub block_id: u64,
}

#[derive(Event, ItemAndAmount)]
pub struct StorageOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

fn spawn_storage(
    camera_query: Query<Entity, With<PlayerCamera>>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
    mut commands: Commands,
) {
    let Some(attribute) = attribute_map.get(&0) else {
        return;
    };
    let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
        return;
    };
    for entity in &camera_query {
        commands.build_screen(
            entity,
            INVENTORY_Y + 1,
            2,
            JustifyContent::End,
            AlignItems::Center,
            |parent| {
                build_panel_grid::<Storage>(parent, 12, 3, Visibility::Hidden, |parent| {
                    for i in 0..36 {
                        build_item::<StorageItem>(parent, 0, 0, i as u8, attribute, atlas);
                    }
                });
            },
        );
    }
}

fn open_storage(
    unload_query: Query<
        (&ItemID, &ItemAmount, &ItemIndex, &BlockID),
        (With<UnloadItem>, Without<StorageItem>),
    >,
    mut item_query: Query<(&mut ItemID, &mut ItemAmount, &ItemIndex), With<StorageItem>>,
    mut event_reader: EventReader<StorageClicked>,
    mut storage_block_id: ResMut<StorageBlockID>,
    mut next_state: ResMut<NextState<UIStates>>,
) {
    for event in event_reader.read() {
        // Items
        for (mut item_id, mut amount, index) in &mut item_query {
            item_id.0 = 0;
            amount.0 = 0;
            for (unload_item_id, unload_amount, unload_index, block_id) in &unload_query {
                if block_id.0 != event.block_id {
                    continue;
                }
                if index.0 != unload_index.0 {
                    continue;
                }
                item_id.0 = unload_item_id.0;
                amount.0 = unload_amount.0;
                break;
            }
        }
        // States
        storage_block_id.0 = event.block_id;
        next_state.set(UIStates::Storage);
    }
    // TODO optimize search unload item
    // TODO spawn when world initialized
    // TODO hash map with block id?
    // TODO enable distance
    // TODO display contents on storage block
}

fn sync_items(
    storage_block_id: Res<StorageBlockID>,
    item_query: Query<
        (&ItemID, &ItemAmount, &ItemIndex),
        (
            With<StorageItem>,
            Or<(Changed<ItemID>, Changed<ItemAmount>)>,
        ),
    >,
    mut unload_query: Query<
        (Entity, &mut ItemID, &mut ItemAmount, &ItemIndex, &BlockID),
        (With<UnloadItem>, Without<StorageItem>),
    >,
    mut commands: Commands,
) {
    for (item_id, amount, index) in &item_query {
        let mut found = false;
        for (entity, mut unload_item_id, mut unload_amount, unload_index, block_id) in
            &mut unload_query
        {
            if block_id.0 != storage_block_id.0 {
                continue;
            }
            if index.0 != unload_index.0 {
                continue;
            }
            if item_id.0 == 0 {
                commands.entity(entity).despawn_recursive();
            } else {
                unload_item_id.0 = item_id.0;
                unload_amount.0 = amount.0;
            }
            found = true;
            break;
        }
        if found {
            continue;
        }
        if item_id.0 == 0 {
            continue;
        }
        commands.spawn((
            UnloadItem,
            BlockID(storage_block_id.0),
            ItemID(item_id.0),
            ItemAmount(amount.0),
            ItemIndex(index.0),
        ));
    }
}

fn destroy_items(
    unload_query: Query<(Entity, &ItemID, &ItemAmount, &BlockID), With<UnloadItem>>,
    mut event_reader: EventReader<BlockDestroied>,
    mut event_writer: EventWriter<ItemDropped>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for (entity, item_id, amount, block_id) in &unload_query {
            if block_id.0 != event.block_id {
                continue;
            }
            if item_id.0 != 0 {
                event_writer.send(ItemDropped {
                    position: event.position,
                    item_id: item_id.0,
                    amount: amount.0,
                });
            }
            commands.entity(entity).despawn_recursive();
        }
    }
    // TODO optimize event mod depends
}

fn destroy_storage(
    mut event_reader: EventReader<BlockDestroied>,
    mut storage_block_id: ResMut<StorageBlockID>,
    mut next_state: ResMut<NextState<UIStates>>,
) {
    for event in event_reader.read() {
        if storage_block_id.0 != event.block_id {
            continue;
        }
        storage_block_id.0 = 0;
        next_state.set(UIStates::None);
    }
}

pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StorageBlockID(0));
        app.add_event::<StorageOverflowed>();
        app.add_event::<StorageClicked>();
        app.add_systems(Startup, spawn_storage);
        app.add_systems(
            Update,
            (
                open_storage,
                sync_items.run_if(in_state(UIStates::Storage)),
                destroy_storage.run_if(in_state(UIStates::Storage)),
            ),
        );
        app.add_systems(
            OnEnter(UIStates::Storage),
            change_visibility::<Storage, Inventory, Inventory>(Visibility::Inherited),
        );
        app.add_systems(
            OnExit(UIStates::Storage),
            change_visibility::<Storage, Inventory, Inventory>(Visibility::Hidden),
        );
        app.add_systems(Last, destroy_items);
    }
}
