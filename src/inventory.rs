use crate::atlas::*;
use crate::camera::*;
use crate::craft::*;
use crate::equipment::*;
use crate::input::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::item_node::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component, Default)]
pub struct Inventory;

#[derive(Component, Default)]
pub struct InventoryItem;

#[derive(Event, ItemAndAmount)]
pub struct InventoryOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

#[derive(Event, ItemAndAmount)]
pub struct InventoryPushedOut {
    pub item_id: u16,
    pub amount: u16,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum InventoryOpened {
    None,
    Opened,
}

pub const INVENTORY_X: u16 = 10;
pub const INVENTORY_Y: u16 = 4;

fn spawn_inventory(
    camera_query: Query<Entity, With<PlayerCamera>>,
    mut commands: Commands,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
) {
    let Some(attribute) = attribute_map.get(&0) else {
        return;
    };
    let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
        return;
    };
    for entity in &camera_query {
        commands.build_screen(
            entity,
            1,
            1,
            JustifyContent::End,
            AlignItems::Center,
            |parent| {
                build_grid::<Inventory>(
                    parent,
                    INVENTORY_X,
                    INVENTORY_Y,
                    Visibility::Hidden,
                    |parent| {
                        for i in 0..INVENTORY_X * INVENTORY_Y {
                            build_item::<InventoryItem>(parent, 0, 0, i as u8, attribute, atlas);
                        }
                    },
                );
            },
        );
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(InventoryOpened::None);
        app.add_event::<InventoryOverflowed>();
        app.add_event::<InventoryPushedOut>();
        app.add_systems(Startup, spawn_inventory);
        app.add_systems(
            Update,
            (
                change_ui_state::<Tab>(UIStates::Inventory).run_if(in_state(UIStates::None)),
                change_ui_state::<Tab>(UIStates::None).run_if(not(in_state(UIStates::None))),
                sync_visibility::<Inventory, InventoryOpened>(
                    InventoryOpened::Opened,
                    InventoryOpened::None,
                ),
            ),
        );
        app.add_systems(
            OnEnter(UIStates::Inventory),
            change_visibility::<Inventory, CraftUI, EquipmentUI>(Visibility::Inherited),
        );
        app.add_systems(
            OnExit(UIStates::Inventory),
            change_visibility::<Inventory, CraftUI, EquipmentUI>(Visibility::Hidden),
        );
    }
}
