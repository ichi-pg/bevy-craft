use bevy::prelude::*;
use bevy::render::texture::TRANSPARENT_IMAGE_HANDLE;
use std::collections::HashMap;

pub struct Atlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AtlasMap(pub HashMap<u8, Atlas>);

fn setup_atlas(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut atlas_map: ResMut<AtlasMap>,
) {
    for (atlas_id, texture, tile_size, columns, rows) in [
        (0, TRANSPARENT_IMAGE_HANDLE, 1, 1, 1),
        (1, asset_server.load("spritesheet_items.png"), 128, 7, 8),
        (2, asset_server.load("spritesheet_tiles.png"), 128, 9, 10),
    ] {
        atlas_map.insert(
            atlas_id,
            Atlas {
                texture,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(tile_size),
                    columns,
                    rows,
                    None,
                    None,
                )),
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
