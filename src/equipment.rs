use crate::atlas::*;
use crate::camera::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::item_node::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct EquipmentUI;

#[derive(Component, Default)]
pub struct EquipmentItem;

#[derive(Event, Default)]
pub struct EquipmentChanged;

fn spawn_equipments(
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
            INVENTORY_Y + 1,
            2,
            JustifyContent::End,
            AlignItems::Center,
            |parent| {
                build_space(parent, INVENTORY_X, 4, JustifyContent::End, |parent| {
                    build_panel_grid::<EquipmentUI>(parent, 3, 4, Visibility::Hidden, |parent| {
                        for i in 0..10 {
                            build_item::<EquipmentItem>(parent, 0, 0, i, attribute, atlas);
                        }
                    });
                });
            },
        );
    }
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
