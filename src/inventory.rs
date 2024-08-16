use crate::input::*;
use crate::item::*;
use crate::ui_states::*;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component, Default)]
pub struct Inventory;

#[derive(Component, Default)]
pub struct InventoryItem;

#[derive(Event, Default, ItemAndAmount)]
pub struct InventoryOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

#[derive(Event, Default, ItemAndAmount)]
pub struct InventoryPushedOut {
    pub item_id: u16,
    pub amount: u16,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum InventoryOpened {
    None,
    Opened,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(InventoryOpened::None);
        app.add_event::<InventoryOverflowed>();
        app.add_event::<InventoryPushedOut>();
        app.add_systems(
            Update,
            (
                change_ui_state::<Tab>(UIStates::Inventory).run_if(in_state(InventoryOpened::None)),
                change_ui_state::<Tab>(UIStates::None).run_if(in_state(InventoryOpened::Opened)),
                sync_visibility::<Inventory, InventoryOpened>(
                    InventoryOpened::Opened,
                    InventoryOpened::None,
                ),
            ),
        );
        app.add_systems(
            OnEnter(UIStates::Inventory),
            change_visibility::<Inventory, Inventory>(Visibility::Inherited),
        );
        app.add_systems(
            OnExit(UIStates::Inventory),
            change_visibility::<Inventory, Inventory>(Visibility::Hidden),
        );
    }
}
