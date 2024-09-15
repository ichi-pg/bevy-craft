use crate::atlas::*;
// use crate::biome::*;
// use crate::biome_id::*;
use crate::block::*;
use crate::item_attribute::*;
use crate::item_id::*;
use crate::math::*;
use crate::random::*;
use bevy::prelude::*;
use noise::*;
use rand::RngCore;

const WORLD_WIDTH: i32 = 200;
const WORLD_HEIGHT: i32 = 100;

fn spawn_world(
    // biome_map: Res<BiomeMap>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
    mut random: ResMut<Random>,
    mut commands: Commands,
) {
    // let order = [
    //     vec![vec![(FOREST_BIOME_ID, CAVE_BIOME_ID)]],
    //     vec![vec![
    //         (MOUNTAIN_BIOME_ID, MINE_BIOME_ID),
    //         (SWAMP_BIOME_ID, DUNGEON_BIOME_ID),
    //     ]],
    //     vec![vec![
    //         (DESERT_BIOME_ID, TEMPLE_BIOME_ID),
    //         (SNOW_BIOME_ID, CRYSTAL_BIOME_ID),
    //         (VOLCANO_BIOME_ID, FORTRESS_BIOME_ID),
    //     ]],
    // ];
    let perlin = Perlin::new(random.next_u32());
    let half_width = WORLD_WIDTH / 2;
    let half_height = WORLD_HEIGHT / 2;
    for x in 0..WORLD_WIDTH {
        let wavelength = perlin
            .get([x as f64
                * perlin
                    .get([x as f64 * 0.01])
                    .normalize_0_1()
                    .interpolate(0.01, 0.005)])
            .normalize_0_1();
        let amplitude = perlin
            .get([x as f64 * 0.01])
            .normalize_0_1()
            .interpolate(half_height as f64, half_height as f64);
        let height: i32 = (wavelength * amplitude) as i32;
        for y in 0..WORLD_HEIGHT {
            if y > height {
                continue;
            }
            commands.build_block(
                SOIL_ITEM_ID,
                (x - half_width) as f32,
                (y - WORLD_HEIGHT) as f32,
                &attribute_map,
                &atlas_map,
                &mut random,
            );
        }
    }
    // TODO high load
}

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}
