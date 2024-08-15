use crate::block::*;
use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_container::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component)]
struct BackgroundItem;

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

#[derive(Event, Default)]
pub struct StorageOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for StorageOverflowed {
    fn item_id(&self) -> u16 {
        self.item_id
    }
    fn amount(&self) -> u16 {
        self.amount
    }
    fn set_item_id(&mut self, item_id: u16) {
        self.item_id = item_id;
    }
    fn set_amount(&mut self, amount: u16) {
        self.amount = amount;
    }
}

fn open_storage(
    mut storage_query: Query<&mut Visibility, Or<(With<Inventory>, With<Storage>)>>,
    mut item_query: Query<(&mut ItemID, &mut ItemAmount, &ItemIndex), With<StorageItem>>,
    background_query: Query<
        (&ItemID, &ItemAmount, &ItemIndex, &BlockID),
        (With<BackgroundItem>, Without<StorageItem>),
    >,
    mut event_reader: EventReader<StorageClicked>,
    mut storage_block_id: ResMut<StorageBlockID>,
    mut next_state: ResMut<NextState<UIStates>>,
) {
    for event in event_reader.read() {
        // Items
        for (mut item_id, mut amount, index) in &mut item_query {
            item_id.0 = 0;
            amount.0 = 0;
            for (background_item_id, background_amount, background_index, block_id) in
                &background_query
            {
                if block_id.0 != event.block_id {
                    continue;
                }
                if index.0 != background_index.0 {
                    continue;
                }
                item_id.0 = background_item_id.0;
                amount.0 = background_amount.0;
                break;
            }
        }
        // States
        for mut visibility in &mut storage_query {
            *visibility = Visibility::Inherited;
        }
        storage_block_id.0 = event.block_id;
        next_state.set(UIStates::Storage);
    }
    // TODO openable when already opened
    // TODO optimize search background item
    // TODO spawn when world initialized
}

fn sync_items(
    item_query: Query<
        (&ItemID, &ItemAmount, &ItemIndex),
        (
            With<StorageItem>,
            Or<(Changed<ItemID>, Changed<ItemAmount>)>,
        ),
    >,
    mut background_query: Query<
        (Entity, &mut ItemID, &mut ItemAmount, &ItemIndex, &BlockID),
        (With<BackgroundItem>, Without<StorageItem>),
    >,
    mut commands: Commands,
    storage_block_id: Res<StorageBlockID>,
) {
    for (item_id, amount, index) in &item_query {
        let mut found = false;
        for (entity, mut background_item_id, mut background_amount, background_index, block_id) in
            &mut background_query
        {
            if block_id.0 != storage_block_id.0 {
                continue;
            }
            if index.0 != background_index.0 {
                continue;
            }
            if item_id.0 == 0 {
                commands.entity(entity).despawn();
            } else {
                background_item_id.0 = item_id.0;
                background_amount.0 = amount.0;
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
            BackgroundItem,
            BlockID(storage_block_id.0),
            ItemID(item_id.0),
            ItemAmount(amount.0),
            ItemIndex(index.0),
        ));
    }
}

fn close_storage(
    mut storage_query: Query<&mut Visibility, Or<(With<Storage>, With<Inventory>)>>,
    input: Res<Input>,
    mut storage_block_id: ResMut<StorageBlockID>,
    mut next_state: ResMut<NextState<UIStates>>,
) {
    if !input.tab && !input.escape {
        return;
    }
    for mut visibility in &mut storage_query {
        *visibility = Visibility::Hidden;
    }
    storage_block_id.0 = 0;
    next_state.set(UIStates::None);
}

fn destroy_storage(
    mut storage_query: Query<&mut Visibility, Or<(With<Inventory>, With<Storage>)>>,
    background_query: Query<(Entity, &ItemID, &ItemAmount, &BlockID), With<BackgroundItem>>,
    mut event_reader: EventReader<BlockDestroied>,
    mut event_writer: EventWriter<ItemDropped>,
    mut commands: Commands,
    mut storage_block_id: ResMut<StorageBlockID>,
) {
    for event in event_reader.read() {
        // Items
        for (entity, item_id, amount, block_id) in &background_query {
            if block_id.0 != event.block_id {
                continue;
            }
            if item_id.0 != 0 {
                event_writer.send(ItemDropped {
                    translation: event.translation,
                    item_id: item_id.0,
                    amount: amount.0,
                });
            }
            commands.entity(entity).despawn();
        }
        // States
        if storage_block_id.0 == event.block_id {
            for mut visibility in &mut storage_query {
                *visibility = Visibility::Hidden;
            }
            storage_block_id.0 = 0;
        }
    }
    // TODO optimize event depends
}

pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StorageBlockID(0));
        app.add_event::<StorageOverflowed>();
        app.add_event::<StorageClicked>();
        app.add_systems(
            Update,
            (
                open_storage.run_if(in_state(UIStates::None)),
                sync_items.run_if(in_state(UIStates::Storage)),
                close_storage.run_if(in_state(UIStates::Storage)),
            ),
        );
        app.add_systems(Last, destroy_storage);
    }
}
