use crate::atlas::*;
use crate::block::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::random::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tree;

fn update_tree(
    query: Query<(&Transform, &ItemID), With<Tree>>,
    atlas_map: Res<AtlasMap>,
    attribute_map: Res<ItemAttributeMap>,
    mut block_map: ResMut<PlacedBlockMap>,
    mut random: ResMut<Random>,
    mut commands: Commands,
) {
    for (transform, item_id) in &query {
        let point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        let tree_power = if let Some(block) = block_map.get(&point) {
            block.tree_power
        } else {
            0
        };
        if tree_power == 0 {
            continue;
        }
        if let Some(block) = block_map.get_mut(&point) {
            block.tree_power = 0;
        };
        let top_point = point + I16Vec2::Y;
        if block_map.contains_key(&top_point) {
            continue;
        }
        commands.build_block(
            item_id.0,
            top_point,
            &attribute_map,
            &atlas_map,
            &mut random,
        );
        block_map.insert(
            top_point,
            PlacedBlock {
                item_id: item_id.0,
                liquid_level: 0,
                tree_power: tree_power - 1,
            },
        );
    }
    // TODO freeze
    // TODO leaf
}

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tree);
    }
}
