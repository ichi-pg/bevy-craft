use crate::equipment::*;
use crate::item::*;
use crate::player::*;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component)]
struct ItemStats;

#[derive(Component, Stats)]
pub struct Health(pub f32);

#[derive(Component, Stats)]
pub struct MaxHealth(pub f32);

#[derive(Component, Stats)]
pub struct PickaxePower(pub f32);

#[derive(Component, Stats)]
pub struct MeleePower(pub f32);

pub trait Stats {
    fn get(&self) -> f32;
    fn set(&mut self, stats: f32);
}

fn spawn_stats(mut commands: Commands) {
    for item in [(101, 20.0)] {
        commands.spawn((ItemStats, ItemID(item.0), Health(item.1)));
    }
    // TODO merge craft materials and stats?
    // TODO item name
}

fn sync_stats<T: Component + Stats>(
    init_value: f32,
) -> impl FnMut(
    Query<&ItemID, With<EquipmentItem>>,
    Query<(&ItemID, &T), With<ItemStats>>,
    Query<&mut T, (With<Player>, Without<ItemStats>)>,
    EventReader<EquipmentChanged>,
) {
    move |equipment_query, query, mut player_query, event_reader| {
        if event_reader.is_empty() {
            return;
        }
        let mut value = init_value;
        for equipment_item_id in &equipment_query {
            for (item_id, stats) in &query {
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

pub struct ItemStatsPlugin;

impl Plugin for ItemStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stats);
        app.add_systems(Update, sync_stats::<MaxHealth>(100.0));
    }
}
