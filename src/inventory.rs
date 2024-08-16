use crate::input::*;
use crate::item::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Inventory;

#[derive(Component, Default)]
pub struct InventoryItem;

#[derive(Event, Default)]
pub struct InventoryOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for InventoryOverflowed {
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

#[derive(Event, Default)]
pub struct InventoryPushedOut {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for InventoryPushedOut {
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

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
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
