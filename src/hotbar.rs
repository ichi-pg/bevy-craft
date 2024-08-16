use crate::item::*;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component, Default)]
pub struct Hotbar;

#[derive(Component, Default)]
pub struct HotbarItem;

#[derive(Event, Default, ItemAndAmount)]
pub struct HotbarOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

#[derive(Event, Default, ItemAndAmount)]
pub struct HotbarPushedOut {
    pub item_id: u16,
    pub amount: u16,
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HotbarOverflowed>();
        app.add_event::<HotbarPushedOut>();
    }
}
