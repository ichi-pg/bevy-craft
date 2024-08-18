use crate::hotbar::*;
use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_dragging::*;
use crate::storage::*;
use crate::ui_states::*;
use bevy::prelude::*;

fn put_in_item<T: Event + ItemAndAmount, U: Component, V: Event + Default + ItemAndAmount>(
    mut query: Query<(Entity, &mut ItemID, &mut ItemAmount), With<U>>,
    mut event_reader: EventReader<T>,
    mut event_writer: EventWriter<V>,
) {
    for event in event_reader.read() {
        // Merge
        let mut found = false;
        let mut empty = None;
        for (entity, item_id, mut amount) in &mut query {
            if item_id.0 == event.item_id() {
                amount.0 += event.amount();
                found = true;
                break;
            }
            if empty.is_none() && item_id.0 == 0 {
                empty = Some(entity);
            }
        }
        if found {
            continue;
        }
        match empty {
            // Overwrite
            Some(entity) => match query.get_mut(entity) {
                Ok((_, mut item_id, mut amount)) => {
                    item_id.0 = event.item_id();
                    amount.0 = event.amount();
                }
                Err(_) => todo!(),
            },
            // Overflow
            None => {
                let mut e: V = V::default();
                e.set_item_id(event.item_id());
                e.set_amount(event.amount());
                event_writer.send(e);
            }
        }
    }
    // TODO refactor V::from
    // TODO which player?
    // TODO max item amount
}

fn push_out_item<T: Component, U: Event + Default + ItemAndAmount>(
    mut query: Query<(&Interaction, &mut ItemID, &mut ItemAmount), (With<T>, Changed<Interaction>)>,
    shift: Res<Shift>,
    mut event_writer: EventWriter<U>,
) {
    if !shift.pressed {
        return;
    }
    for (intersection, mut item_id, mut amount) in &mut query {
        if item_id.0 == 0 {
            continue;
        }
        match intersection {
            Interaction::Pressed => {
                let mut e: U = U::default();
                e.set_item_id(item_id.0);
                e.set_amount(amount.0);
                event_writer.send(e);
                item_id.0 = 0;
                amount.0 = 0;
            }
            Interaction::Hovered => continue,
            Interaction::None => continue,
        }
    }
}

fn bulk_push_out<
    T: Component,
    U: Event + Default + ItemAndAmount,
    V: Component,
    W: Resource + Pressed,
>(
    mut query: Query<(&mut ItemID, &mut ItemAmount), With<T>>,
    filter_query: Query<&ItemID, (With<V>, Without<T>)>,
    pressed: Res<W>,
    shift: Res<Shift>,
    mut event_writer: EventWriter<U>,
) {
    if !pressed.just_pressed() {
        return;
    }
    for (mut item_id, mut amount) in &mut query {
        if item_id.0 == 0 {
            continue;
        }
        if !shift.pressed {
            let mut found = false;
            for filter_item_id in &filter_query {
                if filter_item_id.0 == item_id.0 {
                    found = true;
                    break;
                }
            }
            if !found {
                continue;
            }
        }
        let mut e: U = U::default();
        e.set_item_id(item_id.0);
        e.set_amount(amount.0);
        event_writer.send(e);
        item_id.0 = 0;
        amount.0 = 0;
    }
    // TODO around storages
    // TODO check overflow
    // TODO protected items
    // TODO supply hotbar
    // TODO hash set with item id?
    // TODO button?
}

pub struct ItemPuttingPlugin;

impl Plugin for ItemPuttingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                put_in_item::<ItemPickedUp, HotbarItem, HotbarOverflowed>,
                put_in_item::<HotbarOverflowed, InventoryItem, InventoryOverflowed>,
                put_in_item::<StorageOverflowed, InventoryItem, InventoryOverflowed>,
                (
                    put_in_item::<HotbarPushedOut, StorageItem, StorageOverflowed>,
                    put_in_item::<InventoryPushedOut, StorageItem, StorageOverflowed>,
                )
                    .run_if(in_state(UIStates::Storage)),
                (
                    put_in_item::<HotbarPushedOut, InventoryItem, InventoryOverflowed>,
                    put_in_item::<InventoryPushedOut, HotbarItem, HotbarOverflowed>,
                )
                    .run_if(not(in_state(UIStates::Storage))),
                (
                    push_out_item::<HotbarItem, HotbarPushedOut>,
                    push_out_item::<InventoryItem, InventoryPushedOut>,
                    push_out_item::<StorageItem, StorageOverflowed>,
                )
                    .run_if(in_state(ItemDragged::None))
                    .run_if(in_state(InventoryOpened::Opened)),
                (
                    bulk_push_out::<InventoryItem, InventoryPushedOut, StorageItem, KeyR>,
                    bulk_push_out::<StorageItem, StorageOverflowed, InventoryItem, KeyF>,
                )
                    .run_if(in_state(UIStates::Storage)),
            ),
        );
    }
}
