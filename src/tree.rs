use crate::atlas::*;
use crate::block::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::math::GetOrDefault;
use crate::random::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct Tree;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct TreeMap(pub HashMap<I16Vec2, u8>);

fn update_tree(
    query: Query<(&Transform, &ItemID), With<Tree>>,
    atlas_map: Res<AtlasMap>,
    attribute_map: Res<ItemAttributeMap>,
    mut block_set: ResMut<BlockSet>,
    mut tree_map: ResMut<TreeMap>,
    mut random: ResMut<Random>,
    mut commands: Commands,
) {
    for (transform, item_id) in &query {
        let point: I16Vec2 = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        if !tree_map.contains_key(&point) {
            continue;
        }
        let tree_power = tree_map.get_or_default(&point);
        if tree_power == 0 {
            continue;
        }
        tree_map.remove(&point);
        let top_point = point + I16Vec2::Y;
        if block_set.contains(&top_point) {
            continue;
        }
        commands.build_block(
            item_id.0,
            top_point,
            &attribute_map,
            &atlas_map,
            &mut random,
        );
        tree_map.insert(top_point, tree_power - 1);
        block_set.insert(top_point);
    }
    // TODO remove filter component
    // TODO freeze
    // TODO leaf
}

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TreeMap::default());
        app.add_systems(Update, update_tree);
    }
}
