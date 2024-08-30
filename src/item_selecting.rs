use crate::hotbar::*;
use crate::input::*;
use crate::item::*;
use crate::item_node::*;
use crate::item_stats::*;
use crate::math::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedIndex(pub u8);

#[derive(Resource, Default)]
pub struct SelectedItem {
    pub item_id: u16,
    pub amount: u16,
    pub stats: ItemStats,
}

fn select_wheel(mut selected: ResMut<SelectedIndex>, wheel: Res<Wheel>) {
    if wheel.0 == 0 {
        return;
    }
    selected.0 = (selected.0 as i8 - wheel.0.signum()).repeat(0, 9) as u8;
}

fn select_digit(mut selected: ResMut<SelectedIndex>, digits: Res<Digits>) {
    for (index, digit) in digits.iter().enumerate() {
        if digit.just_pressed {
            selected.0 = index as u8;
        }
    }
}

fn sync_visibility(
    mut query: Query<(&ItemIndex, &mut Visibility), With<ItemSelector>>,
    selected: Res<SelectedIndex>,
) {
    if !selected.is_changed() {
        return;
    }
    for (index, mut visibility) in &mut query {
        *visibility = if index.0 == selected.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        }
    }
}

fn sync_item(
    query: Query<(&ItemID, &ItemAmount, &ItemIndex), With<HotbarItem>>,
    stats_map: Res<ItemStatsMap>,
    selected_index: Res<SelectedIndex>,
    mut selected_item: ResMut<SelectedItem>,
    event_reader: EventReader<HotbarChanged>,
) {
    if !selected_index.is_changed() && event_reader.is_empty() {
        return;
    }
    for (item_id, amount, index) in &query {
        if index.0 != selected_index.0 {
            continue;
        }
        selected_item.item_id = item_id.0;
        selected_item.amount = amount.0;
        selected_item.stats = match stats_map.get(&item_id.0) {
            Some(stats) => stats.clone(),
            None => ItemStats::default(),
        }
    }
}

pub struct ItemSelectingPlugin;

impl Plugin for ItemSelectingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedIndex(0));
        app.insert_resource(SelectedItem::default());
        app.add_systems(
            Update,
            (
                select_wheel.run_if(not(in_state(UIStates::Minimap))),
                select_digit,
                sync_visibility,
                sync_item,
            ),
        );
    }
}
