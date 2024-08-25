use crate::camera::*;
use crate::item::*;
use crate::item_node::*;
use crate::ui_parts::*;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component, Default)]
pub struct Hotbar;

#[derive(Component, Default, SelectableItem)]
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

fn spawn_hotbar(camera_query: Query<Entity, With<PlayerCamera>>, commands: Commands) {
    match camera_query.get_single() {
        Ok(entity) => build_axis_grid::<Hotbar>(
            commands,
            entity,
            0,
            0,
            AlignItems::Center,
            10,
            1,
            Visibility::Inherited,
            |parent| {
                for i in 0..10 {
                    build_item::<HotbarItem>(parent, 0, 0, i as u8);
                }
            },
        ),
        Err(_) => todo!(),
    }
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HotbarOverflowed>();
        app.add_event::<HotbarPushedOut>();
        app.add_systems(Startup, spawn_hotbar);
    }
}
