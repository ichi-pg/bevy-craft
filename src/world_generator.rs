use crate::block::*;
use crate::chunk::*;
use crate::item_attribute::*;
use crate::item_id::*;
use crate::math::*;
use crate::minimap::*;
use crate::random::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;
use bevy::render::texture::*;
use image::RgbaImage;
use noise::*;
use rand::RngCore;
use std::path::Path;

pub const WORLD_WIDTH: i16 = 3500;
const WORLD_HEIGHT: i16 = 1800;

const UNDERGROUND_HEIGHT: i16 = WORLD_HEIGHT * 4 / 5;
pub const ABOVE_GROUND_HEIGHT: i16 = WORLD_HEIGHT / 5;
pub const SURFACE_HEIGHT: i16 = ABOVE_GROUND_HEIGHT / 3;

const HALF_WORLD_WIDTH: i16 = WORLD_WIDTH / 2;
const INVERTED_HALF_WORLD_WIDTH: f64 = 1.0 / HALF_WORLD_WIDTH as f64;
const INVERTED_UNDERGROUND_HEIGHT: f64 = 1.0 / (UNDERGROUND_HEIGHT + SURFACE_HEIGHT) as f64;

const MINIMAP_TEXTURE_NAME: &str = "minimap.png";

fn spawn_world(
    asset_server: Res<AssetServer>,
    attribute_map: Res<ItemAttributeMap>,
    mut random: ResMut<Random>,
    mut unload_blocks_map: ResMut<UnloadBlocksMap>,
    mut placed_block_map: ResMut<PlacedBlockMap>,
    mut commands: Commands,
) {
    let seed = random.next_u32();
    let surface_fbm = Fbm::<Perlin>::new(seed).set_frequency(0.005);
    let cave_fbm = Fbm::<Perlin>::new(seed).set_frequency(0.05);
    let hole_fbm = Fbm::<Perlin>::new(seed).set_frequency(0.01);
    let water_fbm = Fbm::<Perlin>::new(seed + 1).set_frequency(0.02);
    let tree_fbm = Fbm::<Perlin>::new(seed + 1).set_frequency(0.05);
    let ore_fbm = Fbm::<Perlin>::new(seed + 1).set_frequency(0.2);
    let mut imgbuf = RgbaImage::new(WORLD_WIDTH as u32, WORLD_HEIGHT as u32);
    for x in 0..WORLD_WIDTH {
        let fx: f64 = x as f64;
        let farness = (fx - HALF_WORLD_WIDTH as f64) * INVERTED_HALF_WORLD_WIDTH;
        let surface_noise = surface_fbm.get([fx, 0.0]) * SURFACE_HEIGHT as f64;
        let surface = UNDERGROUND_HEIGHT + (surface_noise * farness.abs().powf(0.2)) as i16;
        for y in 0..=surface {
            let fy = y as f64;
            let deepness = fy * INVERTED_UNDERGROUND_HEIGHT;
            // cave
            let cave_noise = cave_fbm.get([fx, fy]);
            if cave_noise > 0.1 + deepness * 0.2 {
                continue;
            }
            // hole
            let noise = hole_fbm.get([fx, fy]);
            if noise > 0.4 + deepness * 0.2 {
                continue;
            }
            // water
            let noise = water_fbm.get([fx, fy]);
            let item_id = if noise > 0.4 - deepness * 0.2 {
                WATER_ITEM_ID
            } else {
                // ore
                let deepness_noise = deepness - cave_noise * cave_noise;
                let noise = ore_fbm.get([fx, fy]);
                if noise > 0.3 && deepness_noise > 0.05 {
                    if deepness_noise < 0.3 {
                        GOLD_ITEM_ID
                    } else if deepness_noise < 0.55 {
                        IRON_ITEM_ID
                    } else if deepness_noise < 0.8 {
                        BRONZE_ITEM_ID
                    } else {
                        COAL_ITEM_ID
                    }
                } else {
                    // underground
                    if deepness_noise < 0.05 {
                        MAGMA_ITEM_ID
                    } else if deepness_noise < 0.3 {
                        GRANITE_ITEM_ID
                    } else if deepness_noise < 0.55 {
                        DEEPSLATE_ITEM_ID
                    } else if deepness_noise < 0.8 {
                        STONE_ITEM_ID
                    } else {
                        // biome
                        let noise = farness + cave_noise * 0.1;
                        if noise < -0.6 {
                            SNOW_ITEM_ID
                        } else if noise < -0.2 {
                            GRASS_ITEM_ID
                        } else if noise > 0.6 {
                            LAVA_ITEM_ID
                        } else if noise > 0.2 {
                            SAND_ITEM_ID
                        } else {
                            SOIL_ITEM_ID
                        }
                    }
                }
            };
            // tree
            macro_rules! placement_block {
                ( $item_id:ident, $x:ident, $y:ident ) => {
                    let Some(attribute) = attribute_map.get(&$item_id) else {
                        todo!()
                    };
                    let point = I16Vec2::new($x - HALF_WORLD_WIDTH, $y - UNDERGROUND_HEIGHT);
                    let chunk_point = point / CHUKN_LENGTH;
                    let unload_blocks = unload_blocks_map.get_or_insert(&chunk_point);
                    unload_blocks.push(UnloadBlock {
                        item_id: $item_id,
                        point,
                    });
                    placed_block_map.insert(
                        point,
                        PlacedBlock {
                            item_id: $item_id,
                            pressure: false,
                        },
                    );
                    let pixel = imgbuf.get_pixel_mut($x as u32, (WORLD_HEIGHT - $y - 1) as u32);
                    *pixel = attribute.minimap_color;
                };
            }
            if y == surface && item_id != WATER_ITEM_ID {
                let noise = tree_fbm.get([fx * cave_noise, fy * cave_noise]);
                if noise > 0.2 {
                    let x = x as i16;
                    for y in y + 1..y + 8 {
                        placement_block!(WOOD_ITEM_ID, x, y);
                    }
                }
            }
            placement_block!(item_id, x, y);
        }
    }
    if let Err(_) = imgbuf.save(Path::new("assets").join(MINIMAP_TEXTURE_NAME)) {
        todo!()
    }
    let texture = asset_server.load_with_settings(MINIMAP_TEXTURE_NAME, |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                mipmap_filter: ImageFilterMode::Linear,
                ..default()
            }),
            ..default()
        }
    });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, MINIMAP_ALPHA),
                ..default()
            },
            texture,
            transform: Transform::from_xyz(
                -0.5,
                (WORLD_HEIGHT / 2 - UNDERGROUND_HEIGHT) as f32 - 0.5,
                0.0,
            ),
            ..default()
        },
        MINIMAP_LAYER,
    ));
    macro_rules! placement_block {
        ( $x:expr, $y:expr ) => {
            placed_block_map.insert(
                I16Vec2::new($x, $y),
                PlacedBlock {
                    item_id: 0,
                    pressure: false,
                },
            );
        };
    }
    for x in -1..=WORLD_WIDTH {
        placement_block!(x - HALF_WORLD_WIDTH, -UNDERGROUND_HEIGHT - 1);
    }
    for y in -1..WORLD_HEIGHT {
        for x in [-HALF_WORLD_WIDTH - 1, HALF_WORLD_WIDTH] {
            placement_block!(x, y - UNDERGROUND_HEIGHT);
        }
    }
    // TODO world end lines
    // TODO high mountain
    // TODO underground balance
    // TODO preset structures
}

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}
