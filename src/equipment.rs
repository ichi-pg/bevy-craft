use crate::inventory::*;
use crate::item::*;
use crate::item_node::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct EquipmentUI;

#[derive(Component, Default)]
pub struct EquipmentItem;

#[derive(Event, Default)]
pub struct EquipmentChanged;

fn spawn_equipments(mut commands: Commands) {
    commands
        .spawn(screen_node(INVENTORY_Y + 1, 2, AlignItems::Center))
        .with_children(|parent: &mut ChildBuilder| {
            parent
                .spawn(grid_space(INVENTORY_X, 4, JustifyContent::End))
                .with_children(|parent| {
                    parent
                        .spawn((grid_node(3, 4, Visibility::Hidden), EquipmentUI))
                        .with_children(|parent| {
                            for i in 0..10 {
                                build_item::<EquipmentItem>(parent, 0, 0, i, false);
                            }
                        });
                });
        });
    // TODO character preview
    // TODO stats preview
    // TODO ability preview
    // TODO slot categorize
}

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EquipmentChanged>();
        app.add_systems(Startup, spawn_equipments);
        app.add_systems(
            Update,
            sync_changed::<EquipmentItem, ItemID, EquipmentChanged>,
        );
    }
}
