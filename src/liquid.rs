use crate::atlas::*;
use crate::block::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::math::*;
use crate::random::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct Liquid;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct LiquidMap(pub HashMap<I16Vec2, u8>);

fn move_liquid(
    query: Query<(Entity, &Transform, &ItemID), With<Liquid>>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
    mut liquid_map: ResMut<LiquidMap>,
    mut random: ResMut<Random>,
    mut commands: Commands,
) {
    for (entity, transform, item_id) in &query {
        let old_point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        for (new_point, min_liquid_level) in [
            (old_point - I16Vec2::Y, 100),
            (old_point - I16Vec2::X, 0),
            (old_point + I16Vec2::X, 0),
        ] {
            let old_liquid_level = liquid_map.get_or_default(&old_point);
            let new_liquid_level = liquid_map.get_or_default(&new_point);
            if new_liquid_level >= old_liquid_level.max(min_liquid_level) {
                continue;
            }
            if new_liquid_level > 0 {
                if let Some(liquid_level) = liquid_map.get_mut(&new_point) {
                    *liquid_level += 1;
                } else {
                    todo!()
                }
            } else {
                commands.build_block(
                    item_id.0,
                    new_point,
                    &attribute_map,
                    &atlas_map,
                    &mut random,
                );
                liquid_map.insert(new_point, 1);
            }
            if old_liquid_level > 1 {
                if let Some(liquid_level) = liquid_map.get_mut(&old_point) {
                    *liquid_level -= 1;
                } else {
                    todo!()
                }
            } else {
                commands.entity(entity).despawn_recursive();
                liquid_map.remove(&old_point);
            }
        }
    }
    // TODO waterfall
    // TODO river
    // TODO throw tree
    // TODO speed
    // TODO freeze
    // TODO sync minimap
}

fn sync_liquid(
    query: Query<(&Children, &Transform), With<Liquid>>,
    mut child_query: Query<&mut Sprite, With<BlockSprite>>,
    liquid_map: Res<LiquidMap>,
) {
    for (children, transform) in &query {
        let point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        let liquid_level = liquid_map.get_or_default(&point);
        for child in children.iter() {
            let Ok(mut sprite) = child_query.get_mut(*child) else {
                todo!()
            };
            sprite.custom_size = Some(Vec2::new(
                BLOCK_SIZE,
                BLOCK_SIZE * liquid_level as f32 * 0.01,
            ));
        }
    }
    // FIXME panic when spawn or despawn block neary liquid level = 1
}

pub struct LiquidPlugin;

impl Plugin for LiquidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LiquidMap::default());
        app.add_systems(Update, sync_liquid);
        app.add_systems(Last, move_liquid);
    }
}
