use crate::atlas::*;
use crate::block::*;
use crate::hit_test::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::math::*;
use crate::player::*;
use crate::random::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct InChunk;

#[derive(Resource, Deref, DerefMut)]
pub struct ChunkPoint(I16Vec2);

#[derive(Event)]
struct ChunkChanged;

pub struct UnloadBlock {
    pub item_id: u16,
    pub point: I16Vec2,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct UnloadBlocksMap(pub HashMap<I16Vec2, Vec<UnloadBlock>>);

impl GetOrInsert<I16Vec2, Vec<UnloadBlock>> for HashMap<I16Vec2, Vec<UnloadBlock>> {
    fn get_or_insert(&mut self, key: &I16Vec2) -> &mut Vec<UnloadBlock> {
        if !self.contains_key(key) {
            self.insert(*key, Vec::new());
        }
        let Some(unload_blocks) = self.get_mut(key) else {
            todo!();
        };
        unload_blocks
    }
}

pub const CHUKN_LENGTH: i16 = 20;
const CHUNK_SIZE: f32 = BLOCK_SIZE * CHUKN_LENGTH as f32;
const INVERTED_CHUNK_SIZE: f32 = 1.0 / CHUNK_SIZE;
const OUTER_SHAPE: Shape = Shape::Rect(Vec2::splat(CHUNK_SIZE * OUTER_LENGTH as f32));
const OUTER_LENGTH: i16 = 1;

fn start_chunk(mut chunk_point: ResMut<ChunkPoint>, mut event_writer: EventWriter<ChunkChanged>) {
    chunk_point.0 = (PLAYER_RESPAWN_POSITION.xy() * INVERTED_CHUNK_SIZE)
        .round()
        .as_i16vec2();
    event_writer.send(ChunkChanged);
}

fn player_moved(
    query: Query<&Transform, (With<PlayerController>, Changed<Transform>)>,
    mut chunk_point: ResMut<ChunkPoint>,
    mut event_writer: EventWriter<ChunkChanged>,
) {
    for transform in &query {
        let new_point = (transform.translation.xy() * INVERTED_CHUNK_SIZE)
            .round()
            .as_i16vec2();
        if new_point == chunk_point.0 {
            continue;
        }
        chunk_point.0 = new_point;
        event_writer.send(ChunkChanged);
    }
    // FIXME blocks respawned even when player moved in same chunk
}

fn without_block(
    query: Query<(Entity, &Transform), (With<Shape>, Without<Block>)>,
    event_reader: EventReader<ChunkChanged>,
    chunk_point: Res<ChunkPoint>,
    mut commands: Commands,
) {
    if event_reader.is_empty() {
        return;
    }
    let chunk_position = chunk_point.as_vec2() * CHUNK_SIZE;
    for (entity, transform) in &query {
        if point_test(transform.translation.xy(), chunk_position, OUTER_SHAPE) {
            commands.entity(entity).insert(InChunk);
        } else {
            commands.entity(entity).remove::<InChunk>();
        }
    }
    // TODO unload
}

fn with_block(
    query: Query<(Entity, &Transform, &ItemID), (With<Shape>, With<Block>)>,
    event_reader: EventReader<ChunkChanged>,
    chunk_point: Res<ChunkPoint>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
    mut unload_blocks_map: ResMut<UnloadBlocksMap>,
    mut commands: Commands,
    mut random: ResMut<Random>,
) {
    if event_reader.is_empty() {
        return;
    }
    let chunk_position = chunk_point.as_vec2() * CHUNK_SIZE;
    for (entity, transform, item_id) in &query {
        let position = transform.translation.xy();
        if point_test(position, chunk_position, OUTER_SHAPE) {
            continue;
        }
        let chunk_point = (position * INVERTED_CHUNK_SIZE).round().as_i16vec2();
        let unload_blocks = unload_blocks_map.get_or_insert(&chunk_point);
        unload_blocks.push(UnloadBlock {
            item_id: item_id.0,
            point: (position * INVERTED_BLOCK_SIZE).as_i16vec2(),
        });
        commands.entity(entity).despawn_recursive();
    }
    for x in -OUTER_LENGTH..=OUTER_LENGTH {
        for y in -OUTER_LENGTH..=OUTER_LENGTH {
            let chunk_point = chunk_point.0 + I16Vec2::new(x, y);
            let unload_blocks = unload_blocks_map.get_or_insert(&chunk_point);
            for block in unload_blocks.iter() {
                commands.build_block(
                    block.item_id,
                    block.point,
                    &attribute_map,
                    &atlas_map,
                    &mut random,
                );
            }
            unload_blocks.clear();
        }
    }
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkPoint(I16Vec2::ZERO));
        app.insert_resource(UnloadBlocksMap::default());
        app.add_event::<ChunkChanged>();
        app.add_systems(Startup, start_chunk);
        app.add_systems(Update, (player_moved, without_block));
        app.add_systems(PostUpdate, with_block);
    }
    // TODO render
    // TODO spawn
    // TODO projectile
    // TODO sweep or tree?
}
