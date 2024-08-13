use crate::block::*;
use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_container::*;
use bevy::prelude::*;

#[derive(Component)]
struct BackgroundItem;

#[derive(Component, Default)]
pub struct Chest;

#[derive(Component, Default)]
pub struct ChestItem;

#[derive(Resource)]
struct ChestBlockID(u64);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChestOpened {
    None,
    Opened,
}

#[derive(Event)]
pub struct ChestClicked {
    pub block_id: u64,
}

#[derive(Event, Default)]
pub struct ChestOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for ChestOverflowed {
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

fn open_chest(
    mut chest_query: Query<&mut Visibility, Or<(With<Inventory>, With<Chest>)>>,
    mut item_query: Query<(&mut ItemID, &mut ItemAmount, &ItemIndex), With<ChestItem>>,
    background_query: Query<
        (&ItemID, &ItemAmount, &ItemIndex, &BlockID),
        (With<BackgroundItem>, Without<ChestItem>),
    >,
    mut event_reader: EventReader<ChestClicked>,
    mut chest_block_id: ResMut<ChestBlockID>,
    mut next_state: ResMut<NextState<ChestOpened>>,
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
        for mut visibility in &mut chest_query {
            *visibility = Visibility::Inherited;
        }
        chest_block_id.0 = event.block_id;
        next_state.set(ChestOpened::Opened);
    }
    // TODO openable when already opened
    // TODO optimize search background item
    // TODO spawn when world initialized
}

fn sync_items(
    item_query: Query<
        (&ItemID, &ItemAmount, &ItemIndex),
        (With<ChestItem>, Or<(Changed<ItemID>, Changed<ItemAmount>)>),
    >,
    mut background_query: Query<
        (Entity, &mut ItemID, &mut ItemAmount, &ItemIndex, &BlockID),
        (With<BackgroundItem>, Without<ChestItem>),
    >,
    mut commands: Commands,
    chest_block_id: Res<ChestBlockID>,
) {
    for (item_id, amount, index) in &item_query {
        let mut found = false;
        for (entity, mut background_item_id, mut background_amount, background_index, block_id) in
            &mut background_query
        {
            if block_id.0 != chest_block_id.0 {
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
            BlockID(chest_block_id.0),
            ItemID(item_id.0),
            ItemAmount(amount.0),
            ItemIndex(index.0),
        ));
    }
}

fn close_chest(
    mut chest_query: Query<&mut Visibility, With<Chest>>,
    input: Res<Input>,
    mut chest_block_id: ResMut<ChestBlockID>,
    mut next_state: ResMut<NextState<ChestOpened>>,
) {
    if !input.tab {
        return;
    }
    for mut visibility in &mut chest_query {
        *visibility = Visibility::Hidden;
    }
    chest_block_id.0 = 0;
    next_state.set(ChestOpened::None);
}

fn destroy_chest(
    mut chest_query: Query<&mut Visibility, Or<(With<Inventory>, With<Chest>)>>,
    background_query: Query<(Entity, &ItemID, &ItemAmount, &BlockID), With<BackgroundItem>>,
    mut event_reader: EventReader<BlockDestroied>,
    mut event_writer: EventWriter<ItemDropped>,
    mut commands: Commands,
    mut chest_block_id: ResMut<ChestBlockID>,
    mut next_state: ResMut<NextState<ChestOpened>>,
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
        if chest_block_id.0 == event.block_id {
            for mut visibility in &mut chest_query {
                *visibility = Visibility::Hidden;
            }
            chest_block_id.0 = 0;
            next_state.set(ChestOpened::None);
        }
    }
    // TODO optimize event depends
}

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChestBlockID(0));
        app.insert_state(ChestOpened::None);
        app.add_event::<ChestOverflowed>();
        app.add_event::<ChestClicked>();
        app.add_systems(
            Update,
            (
                open_chest.run_if(in_state(ChestOpened::None)),
                sync_items.run_if(in_state(ChestOpened::Opened)),
                close_chest.run_if(in_state(ChestOpened::Opened)),
            ),
        );
        app.add_systems(PostUpdate, destroy_chest);
    }
}
