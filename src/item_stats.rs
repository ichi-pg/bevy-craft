use crate::equipment::*;
use crate::hotbar::*;
use crate::item::*;
use crate::item_node::*;
use crate::item_selecting::*;
use crate::player::*;
use crate::stats::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct ItemStats {
    pub max_health: f32,
    pub pickaxe_power: f32,
    pub attack_power: f32,
    pub attack_speed: f32,
    pub move_speed: f32,
    pub jump_power: f32,
}

#[derive(Resource, Deref, DerefMut, Default)]
struct ItemStatsMap(HashMap<u16, ItemStats>);

fn spawn_stats(mut stats_map: ResMut<ItemStatsMap>) {
    for item in [(101, 100.0)] {
        stats_map.insert(
            item.0,
            ItemStats {
                pickaxe_power: item.1,
                ..default()
            },
        );
    }
    for item in [(104, 10.0)] {
        stats_map.insert(
            item.0,
            ItemStats {
                attack_power: item.1,
                ..default()
            },
        );
    }
    // TODO item name
}

fn sync_equipment<T: Component + Stats>(
    init_value: f32,
) -> impl FnMut(
    Query<&ItemID, With<EquipmentItem>>,
    Query<&mut T, With<PlayerController>>,
    EventReader<EquipmentChanged>,
    Res<ItemStatsMap>,
) {
    move |equipment_query, mut player_query, event_reader, stats_map| {
        if event_reader.is_empty() {
            return;
        }
        let mut value = init_value;
        for equipment_item_id in &equipment_query {
            match stats_map.get(&equipment_item_id.0) {
                Some(stats) => value += T::get_item_stats(stats),
                None => continue,
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
    Query<&mut T, With<PlayerController>>,
    Res<SelectedItem>,
    EventReader<HotbarChanged>,
    Res<ItemStatsMap>,
) {
    move |hotbar_query, mut player_query, selected, event_reader, stats_map| {
        if !selected.is_changed() && event_reader.is_empty() {
            return;
        }
        let mut value = init_value;
        for (hotbar_item_id, index) in &hotbar_query {
            if index.0 != selected.0 {
                continue;
            }
            match stats_map.get(&hotbar_item_id.0) {
                Some(stats) => value += T::get_item_stats(stats),
                None => continue,
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
        app.insert_resource(ItemStatsMap::default());
        app.add_systems(Startup, spawn_stats);
        app.add_systems(
            Update,
            (
                sync_equipment::<MaxHealth>(PLAYER_HEALTH),
                sync_selected::<PickaxePower>(PLAYER_PICKAXE_POWER),
                sync_selected::<AttackPower>(0.0),
            ),
        );
    }
}
