use crate::block::*;
use crate::item::*;
use crate::math::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Liquid;

const OFFSET: [I16Vec2; 3] = [I16Vec2::new(0, -1), I16Vec2::new(-1, 0), I16Vec2::new(1, 0)];

fn update_liquid(
    mut query: Query<(&mut Transform, &ItemID), With<Liquid>>,
    mut block_map: ResMut<PlacedBlockMap>,
) {
    for (mut transform, item_id) in &mut query {
        let old_point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        for offset in OFFSET {
            let new_point = old_point + offset;
            if block_map.contains_key(&new_point) {
                continue;
            }
            transform.translation = new_point.as_vec3() * BLOCK_SIZE;
            block_map.remove(&old_point);
            block_map.insert(new_point, PlacedBlock { item_id: item_id.0 });
            break;
        }
    }
    // FIXME non stop
    // TODO speed
    // TODO freeze
    // TODO quarter block
    // TODO sync minimap
}

pub struct LiquidPlugin;

impl Plugin for LiquidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_liquid);
    }
}
