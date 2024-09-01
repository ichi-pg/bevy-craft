use crate::atlas::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct ItemAttribute {
    pub name_id: u16,
    pub description_id: u16,
    pub atlas_id: u8,
    pub atlas_index: u8,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ItemAttributeMap(pub HashMap<u16, ItemAttribute>);

fn create_attributes() -> ItemAttributeMap {
    let atlas = [(1, 56), (2, 85)];
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
            item_id as u16 + 1,
            ItemAttribute {
                name_id: name_id as u16 + 1,
                description_id: description_id as u16 + 1,
                atlas_id,
                atlas_index: atlas_index as u8,
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
