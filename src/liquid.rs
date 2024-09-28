use crate::atlas::*;
use crate::block::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::random::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Liquid;

fn move_liquid(
    query: Query<(Entity, &Transform, &ItemID), With<Liquid>>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
    mut block_map: ResMut<PlacedBlockMap>,
    mut random: ResMut<Random>,
    mut commands: Commands,
) {
    for (entity, transform, item_id) in &query {
        let old_point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        macro_rules! move_liquid {
            ( $new_point:expr, $old_liquid_level:expr ) => {
                let new_point = $new_point;
                let new_liquid_level = if let Some(block) = block_map.get(&new_point) {
                    if block.item_id == item_id.0 {
                        block.liquid_level
                    } else {
                        u8::MAX
                    }
                } else {
                    0
                };
                if new_liquid_level < $old_liquid_level {
                    if new_liquid_level == 0 {
                        commands.build_block(
                            item_id.0,
                            new_point,
                            &attribute_map,
                            &atlas_map,
                            &mut random,
                        );
                        block_map.insert(
                            new_point,
                            PlacedBlock {
                                item_id: item_id.0,
                                liquid_level: 1,
                                tree_power: 0,
                            },
                        );
                    } else {
                        if let Some(new_block) = block_map.get_mut(&new_point) {
                            new_block.liquid_level += 1;
                        }
                    }
                    if let Some(block) = block_map.get_mut(&old_point) {
                        block.liquid_level -= 1;
                        if block.liquid_level == 0 {
                            commands.entity(entity).despawn_recursive();
                            block_map.remove(&old_point);
                        }
                    }
                }
            };
        }
        macro_rules! move_side {
            ( $new_point:expr ) => {
                let old_liquid_level = if let Some(block) = block_map.get(&old_point) {
                    block.liquid_level
                } else {
                    0
                };
                move_liquid!($new_point, old_liquid_level);
            };
        }
        move_liquid!(old_point - I16Vec2::Y, 100);
        move_side!(old_point - I16Vec2::X);
        move_side!(old_point + I16Vec2::X);
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
    block_map: Res<PlacedBlockMap>,
) {
    for (children, transform) in &query {
        let point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
        let Some(block) = block_map.get(&point) else {
            todo!()
        };
        for child in children.iter() {
            let Ok(mut sprite) = child_query.get_mut(*child) else {
                todo!()
            };
            sprite.custom_size = Some(Vec2::new(
                BLOCK_SIZE,
                BLOCK_SIZE * block.liquid_level as f32 * 0.01,
            ));
        }
    }
    // FIXME panic when spawn or despawn block neary liquid level = 1
}

pub struct LiquidPlugin;

impl Plugin for LiquidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_liquid);
        app.add_systems(Last, move_liquid);
    }
}
