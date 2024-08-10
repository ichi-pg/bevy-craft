use crate::item::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Hotbar;

#[derive(Component, Default)]
pub struct HotbarItem;

#[derive(Event, Default)]
pub struct HotbarOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for HotbarOverflowed {
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
pub struct HotbarPushedOut {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for HotbarPushedOut {
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

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HotbarOverflowed>();
        app.add_event::<HotbarPushedOut>();
    }
}
