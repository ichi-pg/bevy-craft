use crate::item_id::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

const EMPTY_ITEM_OFFSET: u16 = 1;

pub struct LocalText {
    pub text: String,
}

#[derive(Resource, Deref, DerefMut)]
pub struct LocalTextMap(pub HashMap<u16, LocalText>);

fn create_local_texts() -> LocalTextMap {
    let mut texts = HashMap::new();
    for (local_text_id, text) in [
        (WOOD_PICKAXE_ITEM_ID, "Wood pickaxe"),
        (
            WOOD_PICKAXE_ITEM_ID + ITEMS_COUNT,
            "Basic pickaxe made by wood.",
        ),
        (WOOD_SWORD_ITEM_ID, "Wood sword"),
        (
            WOOD_SWORD_ITEM_ID + ITEMS_COUNT,
            "Basic sword made by wood.",
        ),
        (WOOD_ITEM_ID + ITEMS_COUNT, "Wood"),
        (
            WOOD_ITEM_ID + ITEMS_COUNT + BLOCKS_COUNT,
            "Basic material from tree.",
        ),
        (SOIL_ITEM_ID + ITEMS_COUNT, "Soil"),
        (
            SOIL_ITEM_ID + ITEMS_COUNT + BLOCKS_COUNT,
            "Basic material from soil.",
        ),
        (STONE_ITEM_ID + ITEMS_COUNT, "Stone"),
        (
            STONE_ITEM_ID + ITEMS_COUNT + BLOCKS_COUNT,
            "Basic material from stone.",
        ),
        (JUNGLE_ITEM_ID + ITEMS_COUNT, "Grass"),
        (
            JUNGLE_ITEM_ID + ITEMS_COUNT + BLOCKS_COUNT,
            "Basic material from grass.",
        ),
    ] {
        texts.insert(
            local_text_id + EMPTY_ITEM_OFFSET,
            LocalText {
                text: String::from(text),
            },
        );
    }
    LocalTextMap(texts)
    // TODO switch with country code
    // TODO unnecessary item description?
}

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_local_texts());
    }
}
