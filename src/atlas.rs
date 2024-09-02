use crate::item_id::*;
use bevy::prelude::*;
use bevy::render::texture::TRANSPARENT_IMAGE_HANDLE;
use std::collections::HashMap;

const ITEMS_X: u16 = 7;
const BLOCKS_X: u16 = 9;

pub struct Atlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub color: Color,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AtlasMap(pub HashMap<u8, Atlas>);

fn setup_atlas(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut atlas_map: ResMut<AtlasMap>,
) {
    for (atlas_id, texture, tile_size, columns, rows, color) in [
        (
            0,
            TRANSPARENT_IMAGE_HANDLE,
            1,
            1,
            1,
            Color::srgb(0.1, 0.1, 0.1),
        ),
        (
            1,
            asset_server.load("spritesheet_items.png"),
            128,
            ITEMS_X,
            ITEMS_COUNT / ITEMS_X,
            Color::NONE,
        ),
        (
            2,
            asset_server.load("spritesheet_tiles.png"),
            128,
            BLOCKS_X,
            BLOCKS_COUNT / BLOCKS_X,
            Color::NONE,
        ),
    ] {
        atlas_map.insert(
            atlas_id,
            Atlas {
                texture,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(tile_size),
                    columns as u32,
                    rows as u32,
                    None,
                    None,
                )),
                color,
            },
        );
    }
    // TODO xml
}

pub struct AtlasPlugin;

impl Plugin for AtlasPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtlasMap::default());
        app.add_systems(PreStartup, setup_atlas);
    }
}
