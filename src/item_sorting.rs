use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::storage::StorageItem;
use crate::ui_states::*;
use bevy::prelude::*;

fn sort_items<T: Component, U: Resource + Pressed>(
    mut query: Query<(&mut ItemID, &mut ItemAmount), With<T>>,
    pressed: Res<U>,
) {
    if !pressed.just_pressed() {
        return;
    }
    let mut items = Vec::with_capacity(40);
    for (item_id, amount) in &query {
        if item_id.0 == 0 {
            continue;
        }
        items.push((item_id.0, amount.0));
    }
    items.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut iter = query.iter_mut();
    for (sorted_item_id, sorted_amount) in items {
        match iter.next() {
            Some((mut item_id, mut amount)) => {
                item_id.0 = sorted_item_id;
                amount.0 = sorted_amount;
            }
            None => todo!(),
        }
    }
    for (mut item_id, mut amount) in iter {
        item_id.0 = 0;
        amount.0 = 0;
    }
    // TODO button?
}

pub struct ItemSortingPlugin;

impl Plugin for ItemSortingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sort_items::<InventoryItem, KeyT>.run_if(in_state(InventoryOpened::Opened)),
                sort_items::<StorageItem, KeyG>.run_if(in_state(UIStates::Storage)),
            ),
        );
    }
}
