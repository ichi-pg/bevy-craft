use crate::item_id::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct ItemAttribute {
    pub name_id: u16,
    pub description_id: u16,
    pub atlas_id: u8,
    pub atlas_index: u8,
    pub minimap_color: image::Rgba<u8>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ItemAttributeMap(pub HashMap<u16, ItemAttribute>);

pub trait ItemText {
    fn local_text_id(attribute: &ItemAttribute) -> u16;
}

fn create_attributes() -> ItemAttributeMap {
    let colors = HashMap::<u16, image::Rgba<u8>>::from_iter([
        (SOIL_ITEM_ID, image::Rgba([187, 128, 68, 255])),
        (STONE_ITEM_ID, image::Rgba([137, 164, 166, 255])),
        (GRANITE_ITEM_ID, image::Rgba([187, 94, 68, 255])),
        (DEEPSLATE_ITEM_ID, image::Rgba([103, 124, 126, 255])),
        (LAVA_ITEM_ID, image::Rgba([232, 106, 23, 255])),
        (SAND_ITEM_ID, image::Rgba([237, 220, 184, 255])),
        (SNOW_ITEM_ID, image::Rgba([231, 249, 255, 255])),
        (WATER_ITEM_ID, image::Rgba([47, 149, 208, 255])),
        (WOOD_ITEM_ID, image::Rgba([145, 103, 63, 255])),
        (COAL_ITEM_ID, image::Rgba([102, 102, 102, 255])),
        (BRONZE_ITEM_ID, image::Rgba([203, 151, 98, 255])),
        (IRON_ITEM_ID, image::Rgba([203, 216, 217, 255])),
        (GOLD_ITEM_ID, image::Rgba([255, 213, 78, 255])),
    ]);
    let atlas = [
        (0, 1),
        (1, ITEMS_COUNT as usize),
        (2, BLOCKS_COUNT as usize),
    ];
    let mut concat_items = Vec::with_capacity(atlas.iter().fold(0, |sum, x| sum + x.1));
    for (atlas_id, cap) in atlas {
        let len = concat_items.len();
        let mut items = Vec::with_capacity(cap);
        for i in 0..cap {
            items.push((len + i, len * 2 + i, len * 2 + cap + i, atlas_id, i));
        }
        concat_items.extend(items);
    }
    let mut attributes = HashMap::new();
    for (item_id, name_id, description_id, atlas_id, atlas_index) in concat_items {
        attributes.insert(
            item_id as u16,
            ItemAttribute {
                name_id: name_id as u16,
                description_id: description_id as u16,
                atlas_id,
                atlas_index: atlas_index as u8,
                minimap_color: match colors.get(&(item_id as u16)) {
                    Some(color) => *color,
                    None => image::Rgba([255, 255, 255, 255]),
                },
            },
        );
    }
    ItemAttributeMap(attributes)
}

pub struct ItemAttributePlugin;

impl Plugin for ItemAttributePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_attributes());
    }
}
