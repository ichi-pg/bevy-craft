use crate::equipment::*;
use crate::item::*;
use crate::player::*;
use bevy::prelude::*;

#[derive(Component)]
struct ItemStats;

#[derive(Component)]
pub struct Defense(pub u16);

fn spawn_stats(mut commands: Commands) {
    for item in [(101, 10)] {
        commands.spawn((ItemStats, ItemID(item.0), Defense(item.1)));
    }
    // TODO merge craft materials and stats?
}

fn sync_stats(
    equipment_query: Query<&ItemID, With<EquipmentItem>>,
    query: Query<(&ItemID, &Defense), With<ItemStats>>,
    mut player_query: Query<&mut Defense, (With<Player>, Without<ItemStats>)>,
    event_reader: EventReader<EquipmentChanged>,
) {
    if event_reader.is_empty() {
        return;
    }
    for mut player_defence in &mut player_query {
        player_defence.0 = 0;
        for equipment_item_id in &equipment_query {
            for (item_id, defence) in &query {
                if item_id.0 == equipment_item_id.0 {
                    player_defence.0 += defence.0;
                }
            }
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
        app.add_systems(Update, sync_stats);
    }
}
