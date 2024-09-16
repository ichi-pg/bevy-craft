// use crate::biome::*;
// use crate::biome_id::*;
use crate::chunk::UnloadBlock;
use crate::chunk::UnloadBlocks;
use crate::item_id::*;
use crate::math::*;
use crate::random::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;
use noise::*;
use rand::RngCore;

const WORLD_WIDTH: i16 = 200;
const WORLD_HEIGHT: i16 = 100;

fn spawn_world(
    // biome_map: Res<BiomeMap>,
    mut random: ResMut<Random>,
    mut unload_blocks: ResMut<UnloadBlocks>,
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
        let height = (wavelength * amplitude) as i16;
        for y in 0..WORLD_HEIGHT {
            if y > height {
                continue;
            }
            unload_blocks.push(UnloadBlock {
                item_id: SOIL_ITEM_ID,
                point: I16Vec2::new(x - half_width, y - WORLD_HEIGHT),
            });
        }
    }
}

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}
