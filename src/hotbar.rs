use crate::atlas::*;
use crate::camera::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::item_node::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component, Default)]
pub struct Hotbar;

#[derive(Component, Default, SelectableItem)]
pub struct HotbarItem;

#[derive(Event, Default)]
pub struct HotbarChanged;

#[derive(Event, ItemAndAmount)]
pub struct HotbarOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

#[derive(Event, ItemAndAmount)]
pub struct HotbarPushedOut {
    pub item_id: u16,
    pub amount: u16,
}

fn spawn_hotbar(
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
            0,
            0,
            JustifyContent::End,
            AlignItems::Center,
            |parent| {
                build_grid::<Hotbar>(parent, 10, 1, Visibility::Inherited, |parent| {
                    for i in 0..10 {
                        build_item::<HotbarItem>(parent, 0, 0, i as u8, attribute, atlas);
                    }
                });
            },
        );
    }
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HotbarChanged>();
        app.add_event::<HotbarOverflowed>();
        app.add_event::<HotbarPushedOut>();
        app.add_systems(Startup, spawn_hotbar);
        app.add_systems(Update, sync_changed::<HotbarItem, ItemID, HotbarChanged>);
    }
}
