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
pub struct ItemStatsMap(HashMap<u16, ItemStats>);

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

fn sync_stats<T: Component + Stats>(
    init_value: f32,
) -> impl FnMut(
    Query<&ItemID, With<EquipmentItem>>,
    Query<(&ItemID, &ItemIndex), With<HotbarItem>>,
    Query<&mut T, With<PlayerController>>,
    EventReader<EquipmentChanged>,
    EventReader<HotbarChanged>,
    Res<SelectedItem>,
    Res<ItemStatsMap>,
) {
    move |equipment_query,
          hotbar_query,
          mut player_query,
          equipment_event_reader,
          hotbar_event_reader,
          selected,
          stats_map| {
        if !selected.is_changed()
            && equipment_event_reader.is_empty()
            && hotbar_event_reader.is_empty()
        {
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
    // TODO buff
    // TODO debuff
    // TODO enchant
}

pub struct ItemStatsPlugin;

impl Plugin for ItemStatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemStatsMap::default());
        app.add_systems(Startup, spawn_stats);
        app.add_systems(
            Update,
            (
                sync_stats::<MaxHealth>(PLAYER_HEALTH),
                sync_stats::<PickaxePower>(PLAYER_PICKAXE_POWER),
                sync_stats::<AttackPower>(PLAYER_ATTACK_POWER),
                sync_stats::<AttackSpeed>(PLAYER_ATTACK_SPEED),
                sync_stats::<MoveSpeed>(PLAYER_MOVE_SPEED),
                sync_stats::<JumpPower>(PLAYER_JUMP_POWER),
            ),
        );
    }
}
