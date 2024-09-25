use crate::block::*;
use crate::item::*;
use crate::item_id::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Surface;

fn update_soil(
    mut query: Query<(&Transform, &ItemID, &mut TextureAtlas), With<Surface>>,
    block_map: Res<PlacedBlockMap>,
) {
    for (transform, item_id, mut atlas) in &mut query {
        let top_point =
            (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2() + I16Vec2::Y;
        let top_index = match item_id.0 {
            SOIL_ITEM_ID => 43,
            _ => todo!(),
        };
        atlas.index = if let Some(block) = block_map.get(&top_point) {
            match block.item_id {
                WOOD_ITEM_ID => top_index,
                WATER_ITEM_ID => top_index,
                LAVA_ITEM_ID => top_index,
                _ => match item_id.0 {
                    SOIL_ITEM_ID => 52,
                    _ => todo!(),
                },
            }
        } else {
            top_index
        };
    }
    // TODO freeze
    // TODO flower and grass
    // TODO farming
}

pub struct SurfacePlugin;

impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_soil);
    }
}
