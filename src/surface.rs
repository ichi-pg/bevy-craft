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
        let point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        let top_point = point + I16Vec2::Y;
        atlas.index = if block_map.contains_key(&top_point) {
            match item_id.0 {
                SOIL_ITEM_ID => 52,
                _ => todo!(),
            }
        } else {
            match item_id.0 {
                SOIL_ITEM_ID => 43,
                _ => todo!(),
            }
        };
    }
    // TODO when top is tree or liquid
    // TODO freeze
    // TODO flower and grass
    // TODO plow
}

pub struct SurfacePlugin;

impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_soil);
    }
}
