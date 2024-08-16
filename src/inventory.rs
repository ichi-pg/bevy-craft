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

fn sync_opened(
    query: Query<&Visibility, (With<Inventory>, Changed<Visibility>)>,
    mut next_state: ResMut<NextState<InventoryOpened>>,
) {
    for visibility in &query {
        match *visibility {
            Visibility::Inherited => next_state.set(InventoryOpened::Opened),
            Visibility::Hidden => next_state.set(InventoryOpened::None),
            Visibility::Visible => todo!(),
        }
    }
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
                open_ui::<Tab>(UIStates::Inventory).run_if(in_state(InventoryOpened::None)),
                close_ui::<Tab>.run_if(in_state(InventoryOpened::Opened)),
                sync_opened,
            ),
        );
        app.add_systems(
            OnEnter(UIStates::Inventory),
            on_open_ui::<Inventory, Inventory>,
        );
        app.add_systems(
            OnExit(UIStates::Inventory),
            on_close_ui::<Inventory, Inventory>,
        );
    }
}
