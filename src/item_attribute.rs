use bevy::prelude::*;
use std::collections::HashMap;

pub struct ItemAttribute {
    pub name_id: u16,
    pub description_id: u16,
    pub atlas_id: u16,
    pub atlas_coord: Vec2,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ItemAttributeMap(pub HashMap<u16, ItemAttribute>);

fn create_attributes() -> ItemAttributeMap {
    let mut attributes = HashMap::new();
    for item in [(1, 1, 1, 1, Vec2::ZERO)] {
        attributes.insert(
            item.0,
            ItemAttribute {
                name_id: item.1,
                description_id: item.2,
                atlas_id: item.3,
                atlas_coord: item.4,
            },
        );
    }
    ItemAttributeMap(attributes)
    // TODO localization
}

pub struct ItemAttributePlugin;

impl Plugin for ItemAttributePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_attributes());
    }
}
