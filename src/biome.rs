use crate::biome_id::*;
use crate::item_id::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct Biome {
    pub item_id: u16,
}

#[derive(Resource, Deref, DerefMut)]
pub struct BiomeMap(HashMap<u8, Biome>);

fn create_biomes() -> BiomeMap {
    let mut biomes = HashMap::new();
    for (biome_id, item_id) in [
        (FOREST_BIOME_ID, GRASS_ITEM_ID),
        (SWAMP_BIOME_ID, SOIL_ITEM_ID),
        (MOUNTAIN_BIOME_ID, SOIL_ITEM_ID),
        (DESERT_BIOME_ID, SAND_ITEM_ID),
        (SNOW_BIOME_ID, SNOW_ITEM_ID),
        (VOLCANO_BIOME_ID, LAVA_ITEM_ID),
        (CAVE_BIOME_ID, STONE_ITEM_ID),
        (MINE_BIOME_ID, STONE_ITEM_ID),
        (TEMPLE_BIOME_ID, STONE_ITEM_ID),
        (DUNGEON_BIOME_ID, STONE_ITEM_ID),
        (FORTRESS_BIOME_ID, STONE_ITEM_ID),
        (CRYSTAL_BIOME_ID, STONE_ITEM_ID),
    ] {
        biomes.insert(biome_id, Biome { item_id });
    }
    BiomeMap(biomes)
}

pub struct BiomePlugin;

impl Plugin for BiomePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_biomes());
    }
}
