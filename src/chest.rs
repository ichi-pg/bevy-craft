use crate::block::*;
use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
struct BackgroundItem;

#[derive(Component, Default)]
pub struct Chest;

#[derive(Component, Default)]
pub struct ChestItem;

#[derive(Event)]
pub struct ChestClicked {
    pub block_id: u64,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChestOpened {
    None,
    Opened,
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
    mut chest_query: Query<(Entity, &mut Visibility), Or<(With<Inventory>, With<Chest>)>>,
    mut item_query: Query<(&mut ItemID, &mut ItemAmount), With<ChestItem>>,
    background_query: Query<
        (&ItemID, &ItemAmount, &BlockID),
        (With<BackgroundItem>, Without<ChestItem>),
    >,
    mut event_reader: EventReader<ChestClicked>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<ChestOpened>>,
) {
    for event in event_reader.read() {
        for (entity, mut visibility) in &mut chest_query {
            *visibility = Visibility::Inherited;
            commands.entity(entity).insert(BlockID(event.block_id));
        }
        let mut iter = item_query.iter_mut();
        for (background_item_id, background_amount, block_id) in &background_query {
            if block_id.0 != event.block_id {
                continue;
            }
            match iter.next() {
                Some((mut item_id, mut amount)) => {
                    item_id.0 = background_item_id.0;
                    amount.0 = background_amount.0;
                }
                None => todo!(),
            }
        }
        for (mut item_id, mut amount) in iter {
            item_id.0 = 0;
            amount.0 = 0;
        }
        next_state.set(ChestOpened::Opened);
    }
    // TODO openable when already opened
    // TODO efficient background query
    // TODO optimize empty slots
}

fn close_chest(
    mut chest_query: Query<(&mut Visibility, &BlockID), With<Chest>>,
    item_query: Query<(&ItemID, &ItemAmount), With<ChestItem>>,
    mut background_query: Query<
        (&mut ItemID, &mut ItemAmount, &BlockID),
        (With<BackgroundItem>, Without<ChestItem>),
    >,
    mut commands: Commands,
    input: Res<Input>,
    mut next_state: ResMut<NextState<ChestOpened>>,
) {
    if !input.tab {
        return;
    }
    for (mut visibility, chest_block_id) in &mut chest_query {
        *visibility = Visibility::Hidden;
        let mut iter = item_query.iter();
        for (mut background_item_id, mut background_amount, block_id) in &mut background_query {
            if block_id.0 != chest_block_id.0 {
                continue;
            }
            match iter.next() {
                Some((item_id, amount)) => {
                    background_item_id.0 = item_id.0;
                    background_amount.0 = amount.0;
                }
                None => todo!(),
            }
        }
        for (item_id, amount) in iter {
            commands.spawn((
                BackgroundItem,
                BlockID(chest_block_id.0),
                ItemID(item_id.0),
                ItemAmount(amount.0),
            ));
        }
        next_state.set(ChestOpened::None);
    }
}

fn destroy_chest(
    mut query: Query<(Entity, &ItemID, &ItemAmount, &BlockID), With<BackgroundItem>>,
    mut event_reader: EventReader<BlockDestroied>,
    mut event_writer: EventWriter<ItemDropped>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for (entity, item_id, amount, block_id) in &mut query {
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
    }
    // FIXME destroy when opened
}

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ChestOpened::None);
        app.add_event::<ChestOverflowed>();
        app.add_event::<ChestClicked>();
        app.add_systems(
            Update,
            (
                open_chest.run_if(in_state(ChestOpened::None)),
                close_chest.run_if(in_state(ChestOpened::Opened)),
            ),
        );
        app.add_systems(PostUpdate, destroy_chest);
    }
}
