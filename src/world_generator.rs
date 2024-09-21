use crate::chunk::*;
use crate::item_attribute::ItemAttributeMap;
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

const WORLD_WIDTH: i16 = 3500;
const WORLD_HEIGHT: i16 = 1800;

const UNDERGROUND_HEIGHT: i16 = WORLD_HEIGHT * 4 / 5;
pub const SURFACE_HEIGHT: i16 = WORLD_HEIGHT / 3 / 5;

const HALF_WORLD_WIDTH: i16 = WORLD_WIDTH / 2;
const INVERTED_HALF_WORLD_WIDTH: f64 = 1.0 / HALF_WORLD_WIDTH as f64;
const INVERTED_UNDERGROUND_HEIGHT: f64 = 1.0 / (UNDERGROUND_HEIGHT + SURFACE_HEIGHT) as f64;

const MINIMAP_TEXTURE: &str = "minimap.png";

fn spawn_world(
    asset_server: Res<AssetServer>,
    attribute_map: Res<ItemAttributeMap>,
    mut random: ResMut<Random>,
    mut unload_blocks_map: ResMut<UnloadBlocksMap>,
    mut commands: Commands,
) {
    let seed = random.next_u32();
    let fbm = Fbm::<Perlin>::new(seed);
    let fbm2 = Fbm::<Perlin>::new(seed + 1);
    let mut imgbuf = RgbaImage::new(WORLD_WIDTH as u32, WORLD_HEIGHT as u32);
    for x in 0..WORLD_WIDTH {
        let noise = fbm.get([x as f64 * 0.005, 0.0]) * SURFACE_HEIGHT as f64;
        for y in 0..UNDERGROUND_HEIGHT + noise as i16 {
            let distance = (x - HALF_WORLD_WIDTH) as f64 * INVERTED_HALF_WORLD_WIDTH;
            let depth = y as f64 * INVERTED_UNDERGROUND_HEIGHT;
            let base_noise = fbm.get([x as f64 * 0.05, y as f64 * 0.05]);
            if base_noise > 0.1 + depth * 0.2 {
                continue;
            }
            let noise = fbm.get([x as f64 * 0.01, y as f64 * 0.01]);
            if noise > 0.4 + depth * 0.2 {
                continue;
            }
            let noise = fbm2.get([x as f64 * 0.02, y as f64 * 0.02]);
            let item_id = if noise > 0.4 - depth * 0.2 {
                WATER_ITEM_ID
            } else {
                let noise = depth - base_noise.powi(2);
                if noise < 0.1 {
                    LAVA_ITEM_ID
                } else if noise < 0.3 {
                    GRANITE_ITEM_ID
                } else if noise < 0.55 {
                    STONE_ITEM_ID
                } else {
                    let noise = distance + base_noise * 0.1;
                    let abs = noise.abs();
                    if abs < 0.2 {
                        SOIL_ITEM_ID
                    } else if abs < 0.6 {
                        if noise < 0.0 {
                            SAND_ITEM_ID
                        } else {
                            SAND_ITEM_ID
                        }
                    } else {
                        if noise < 0.0 {
                            SNOW_ITEM_ID
                        } else {
                            SNOW_ITEM_ID
                        }
                    }
                }
            };
            let Some(attribute) = attribute_map.get(&item_id) else {
                todo!()
            };
            let point = I16Vec2::new(x - HALF_WORLD_WIDTH, y - UNDERGROUND_HEIGHT);
            let chunk_point = point / CHUKN_LENGTH;
            let unload_blocks = unload_blocks_map.get_or_insert(&chunk_point);
            unload_blocks.push(UnloadBlock { item_id, point });
            let pixel = imgbuf.get_pixel_mut(x as u32, (WORLD_HEIGHT - y - 1) as u32);
            *pixel = attribute.minimap_color;
        }
    }
    if let Err(_) = imgbuf.save(Path::new("assets").join(MINIMAP_TEXTURE)) {
        todo!()
    }
    let texture = asset_server.load_with_settings(MINIMAP_TEXTURE, |s: &mut _| {
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
    // TODO tree
    // TODO ore
    // TODO player spawn point
    // TODO scene
}

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}
