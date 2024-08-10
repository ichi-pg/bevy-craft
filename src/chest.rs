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
) {
    for event in event_reader.read() {
        let mut found = false;
        for (entity, mut visibility) in &mut chest_query {
            if *visibility == Visibility::Inherited {
                found = true;
                continue;
            }
            *visibility = Visibility::Inherited;
            commands.entity(entity).insert(BlockID(event.block_id));
        }
        if found {
            continue;
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
    }
    // TODO openable when already opened
    // TODO efficient background query
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
) {
    if !input.tab {
        return;
    }
    for (mut visibility, chest_block_id) in &mut chest_query {
        if *visibility == Visibility::Hidden {
            continue;
        }
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
    }
}

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChestOverflowed>();
        app.add_event::<ChestClicked>();
        app.add_systems(Update, (open_chest, close_chest));
    }
}
