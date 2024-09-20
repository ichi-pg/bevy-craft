// use crate::biome::*;
// use crate::biome_id::*;
use crate::chunk::*;
use crate::item_id::*;
use crate::math::*;
use crate::random::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;
use image::*;
use noise::*;
use rand::RngCore;

const WORLD_WIDTH: i16 = 8000;
const HALF_WORLD_WIDTH: i16 = WORLD_WIDTH / 2;
const WORLD_HEIGHT: i16 = UNDERGROUND_HEIGHT + SURFACE_HEIGHT + SKY_HEIGHT;
const UNDERGROUND_HEIGHT: i16 = 1600;
pub const SURFACE_HEIGHT: i16 = 200;
const SKY_HEIGHT: i16 = 400;

fn octave_noise_1d(perlin: &Perlin, x: f64, t: f64) -> f64 {
    let mut a = 1.0;
    let mut f = 1.0;
    let mut max_value = 0.0;
    let mut total_value = 0.0;
    let per = 0.5;
    for _ in 0..5 {
        total_value += a * perlin.get([x * f * t]);
        max_value += a;
        a *= per;
        f *= 2.0;
    }
    total_value / max_value
}

fn octave_noise_2d(perlin: &Perlin, x: f64, y: f64, t: f64) -> f64 {
    let mut a = 1.0;
    let mut f = 1.0;
    let mut max_value = 0.0;
    let mut total_value = 0.0;
    let per = 0.5;
    for _ in 0..5 {
        total_value += a * perlin.get([x * f * t, y * f * t]);
        max_value += a;
        a *= per;
        f *= 2.0;
    }
    total_value / max_value
}

fn spawn_world(
    // biome_map: Res<BiomeMap>,
    mut random: ResMut<Random>,
    mut unload_blocks_map: ResMut<UnloadBlocksMap>,
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
    let mut imgbuf = RgbaImage::new(WORLD_WIDTH as u32, WORLD_HEIGHT as u32);
    for x in 0..WORLD_WIDTH {
        let surface_height = SURFACE_HEIGHT as f64 * octave_noise_1d(&perlin, x as f64, 0.001);
        // let surface_height = SURFACE_HEIGHT as f64
        //     * perlin
        //         .get([x as f64
        //             * perlin
        //                 .get([x as f64 * 0.01])
        //                 .normalize_0_1()
        //                 .interpolate(0.0005, 0.0005)])
        //         .normalize_0_1()
        //     * perlin.get([x as f64 * 0.005]).normalize_0_1();
        let point_x = x - HALF_WORLD_WIDTH;
        for y in 0..UNDERGROUND_HEIGHT + surface_height as i16 {
            let noise = octave_noise_2d(&perlin, x as f64, y as f64, 0.05).normalize_0_1();
            if noise <= 0.5 {
                continue;
            }
            let point = I16Vec2::new(point_x, y - UNDERGROUND_HEIGHT);
            let chunk_point = point / CHUKN_LENGTH;
            let unload_blocks = unload_blocks_map.get_or_insert(&chunk_point);
            unload_blocks.push(UnloadBlock {
                item_id: SOIL_ITEM_ID,
                point,
            });
            let pixel = imgbuf.get_pixel_mut(x as u32, (WORLD_HEIGHT - y - 1) as u32);
            *pixel = image::Rgba([187, 128, 68, 255]);
        }
    }
    if let Err(_) = imgbuf.save("assets/generated/minimap.png") {
        todo!()
    }
}

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}
