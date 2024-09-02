use crate::equipment::*;
use crate::item::*;
use crate::item_id::*;
use crate::item_selecting::*;
use crate::player::*;
use crate::stats::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct ItemStats {
    pub max_health: f32,
    pub pickaxe_power: f32,
    pub attack_power: f32,
    pub attack_speed: f32,
    pub move_speed: f32,
    pub jump_power: f32,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ItemStatsMap(HashMap<u16, ItemStats>);

fn create_stats() -> ItemStatsMap {
    let mut stats = HashMap::new();
    for (item_id, pickaxe_power) in [(WOOD_PICKAXE_ID, 100.0)] {
        stats.insert(
            item_id,
            ItemStats {
                pickaxe_power,
                ..default()
            },
        );
    }
    for (item_id, attack_power) in [(WOOD_SWORD_ID, 10.0)] {
        stats.insert(
            item_id,
            ItemStats {
                attack_power,
                ..default()
            },
        );
    }
    ItemStatsMap(stats)
}

fn sync_stats<T: Component + Stats>(
    init_value: f32,
) -> impl FnMut(
    Query<&ItemID, With<EquipmentItem>>,
    Query<&mut T, With<PlayerController>>,
    EventReader<EquipmentChanged>,
    Res<SelectedItem>,
    Res<ItemStatsMap>,
) {
    move |equipment_query, mut player_query, event_reader, selected, stats_map| {
        if !selected.is_changed() && event_reader.is_empty() {
            return;
        }
        let mut value = init_value + T::get_item_stats(&selected.stats);
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
    // TODO skill
}

pub struct ItemStatsPlugin;

impl Plugin for ItemStatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_stats());
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
