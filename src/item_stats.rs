use crate::equipment::*;
use crate::hotbar::*;
use crate::item::*;
use crate::item_node::*;
use crate::item_selecting::*;
use crate::player::*;
use crate::stats::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component)]
struct ItemStats;

#[derive(Event, Default)]
struct HotbarChanged;

fn spawn_stats(mut commands: Commands) {
    for item in [(101, 100.0)] {
        commands.spawn((ItemStats, ItemID(item.0), PickaxePower(item.1)));
    }
    // TODO merge craft materials and stats?
    // TODO item name
}

fn sync_equipment<T: Component + Stats>(
    init_value: f32,
) -> impl FnMut(
    Query<&ItemID, With<EquipmentItem>>,
    Query<(&ItemID, &T), With<ItemStats>>,
    Query<&mut T, (With<PlayerController>, Without<ItemStats>)>,
    EventReader<EquipmentChanged>,
) {
    move |equipment_query, stats_query, mut player_query, event_reader| {
        if event_reader.is_empty() {
            return;
        }
        let mut value = init_value;
        for equipment_item_id in &equipment_query {
            for (item_id, stats) in &stats_query {
                if item_id.0 == equipment_item_id.0 {
                    value += stats.get();
                }
            }
        }
        for mut player_stats in &mut player_query {
            player_stats.set(value);
        }
    }
    // TODO attach defense to equipment item?
    // TODO which player?
    // TODO hash map with item id?
}

fn sync_selected<T: Component + Stats>(
    init_value: f32,
) -> impl FnMut(
    Query<(&ItemID, &ItemIndex), With<HotbarItem>>,
    Query<(&ItemID, &T), With<ItemStats>>,
    Query<&mut T, (With<PlayerController>, Without<ItemStats>)>,
    Res<SelectedItem>,
    EventReader<HotbarChanged>,
) {
    move |hotbar_query, stats_query, mut player_query, selected, event_reader| {
        if !selected.is_changed() && event_reader.is_empty() {
            return;
        }
        let mut value = init_value;
        for (hotbar_item_id, index) in &hotbar_query {
            if index.0 != selected.0 {
                continue;
            }
            for (item_id, stats) in &stats_query {
                if item_id.0 == hotbar_item_id.0 {
                    value += stats.get();
                }
            }
        }
        for mut player_stats in &mut player_query {
            player_stats.set(value);
        }
    }
}

pub struct ItemStatsPlugin;

impl Plugin for ItemStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HotbarChanged>();
        app.add_systems(Startup, spawn_stats);
        app.add_systems(
            Update,
            (
                sync_equipment::<MaxHealth>(PLAYER_HEALTH),
                sync_selected::<PickaxePower>(PLAYER_PICKAXE_POWER),
                sync_changed::<HotbarItem, ItemID, HotbarChanged>,
            ),
        );
    }
}
