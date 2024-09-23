use crate::block::*;
use crate::item::*;
use crate::math::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Liquid;

fn update_liquid(
    mut query: Query<(&mut Transform, &ItemID), With<Liquid>>,
    mut block_map: ResMut<PlacedBlockMap>,
) {
    for (mut transform, item_id) in &mut query {
        let old_point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        macro_rules! move_liquid {
            ( $point:ident ) => {
                transform.translation = $point.as_vec3() * BLOCK_SIZE;
                block_map.remove(&old_point);
                block_map.insert(
                    $point,
                    PlacedBlock {
                        item_id: item_id.0,
                        pressure: false,
                        tree_power: 0,
                    },
                );
                continue;
            };
        }
        // down
        let new_point = old_point - I16Vec2::Y;
        if !block_map.contains_key(&new_point) {
            move_liquid!(new_point);
        };
        // left down
        let left_point = old_point - I16Vec2::X;
        let new_point = left_point - I16Vec2::Y;
        if !block_map.contains_key(&left_point) && !block_map.contains_key(&new_point) {
            move_liquid!(new_point);
        };
        // down right
        let right_point = old_point + I16Vec2::X;
        let new_point = right_point - I16Vec2::Y;
        if !block_map.contains_key(&right_point) && !block_map.contains_key(&new_point) {
            move_liquid!(new_point);
        };
        // top pressure
        let mut pressure = false;
        let top_point = old_point + I16Vec2::Y;
        if let Some(top) = block_map.get(&top_point) {
            pressure = top.item_id == item_id.0;
        };
        // side pressure
        if let Some(side) = block_map.get(&left_point) {
            pressure |= side.pressure && side.item_id == item_id.0;
        };
        if let Some(side) = block_map.get(&right_point) {
            pressure |= side.pressure && side.item_id == item_id.0;
        };
        // side top pressure
        if !block_map.contains_key(&top_point) {
            if let Some(side) = block_map.get(&(left_point + I16Vec2::Y)) {
                pressure |= side.item_id == item_id.0;
            };
            if let Some(side) = block_map.get(&(right_point + I16Vec2::Y)) {
                pressure |= side.item_id == item_id.0;
            };
        }
        if let Some(block) = block_map.get_mut(&old_point) {
            block.pressure = pressure;
        };
        // slide
        if !pressure {
            continue;
        }
        if !block_map.contains_key(&left_point) {
            move_liquid!(left_point);
        };
        if !block_map.contains_key(&right_point) {
            move_liquid!(right_point);
        };
    }
    // TODO liquid level

    // TODO surface tension
    // TODO quarter block
    // TODO merge extra top
    // TODO waterfall
    // TODO water flow

    // TODO throw tree
    // TODO speed
    // TODO freeze
    // TODO sync minimap
}

pub struct LiquidPlugin;

impl Plugin for LiquidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_liquid);
    }
}
