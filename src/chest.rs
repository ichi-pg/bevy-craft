use crate::item::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Chest;

#[derive(Component, Default)]
pub struct ChestItem;

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

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChestOverflowed>();
    }
    // TODO open chest
    // TODO background storage
}
