use std::collections::HashMap;

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

pub const CHUKN_LENGTH: i16 = 20;
const CHUNK_SIZE: f32 = BLOCK_SIZE * CHUKN_LENGTH as f32;
const INNER_SHAPE: Shape = Shape::Rect(Vec2::splat(CHUNK_SIZE));
const OUTER_SHAPE: Shape = Shape::Rect(Vec2::splat(CHUNK_SIZE * 2.0));

fn start_chunk(mut chunk_point: ResMut<ChunkPoint>, mut event_writer: EventWriter<ChunkChanged>) {
    chunk_point.0 = (PLAYER_RESPAWN_POSITION / CHUNK_SIZE).to_i16vec2();
    event_writer.send(ChunkChanged);
}

fn player_moved(
    query: Query<&Transform, (With<PlayerController>, Changed<Transform>)>,
    mut chunk_point: ResMut<ChunkPoint>,
    mut event_writer: EventWriter<ChunkChanged>,
) {
    for transform in &query {
        if point_test(
            transform.translation.xy(),
            chunk_point.to_f32vec2() * CHUNK_SIZE,
            INNER_SHAPE,
        ) {
            return;
        }
        chunk_point.0 = (transform.translation / CHUNK_SIZE).to_i16vec2();
        event_writer.send(ChunkChanged);
    }
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
    let chunk_position = chunk_point.to_f32vec2() * CHUNK_SIZE;
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
    let chunk_position = chunk_point.to_f32vec2() * CHUNK_SIZE;
    for (entity, transform, item_id) in &query {
        if point_test(transform.translation.xy(), chunk_position, OUTER_SHAPE) {
            continue;
        }
        let chunk_point = (transform.translation / CHUNK_SIZE).to_i16vec2();
        if !unload_blocks_map.contains_key(&chunk_point) {
            unload_blocks_map.insert(chunk_point, Vec::new());
        }
        let Some(unload_blocks) = unload_blocks_map.get_mut(&chunk_point) else {
            return;
        };
        unload_blocks.push(UnloadBlock {
            item_id: item_id.0,
            point: (transform.translation / BLOCK_SIZE).to_i16vec2(),
        });
        commands.entity(entity).despawn_recursive();
    }
    for (chunk_point, unload_blocks) in unload_blocks_map.iter_mut() {
        if !point_test(
            chunk_point.to_f32vec2() * CHUNK_SIZE,
            chunk_position,
            OUTER_SHAPE,
        ) {
            continue;
        }
        for block in unload_blocks.iter() {
            if !point_test(
                block.point.to_f32vec2() * BLOCK_SIZE,
                chunk_position,
                OUTER_SHAPE,
            ) {
                continue;
            }
            commands.build_block(
                block.item_id,
                block.point,
                &attribute_map,
                &atlas_map,
                &mut random,
            );
        }
        unload_blocks.retain(|block| {
            !point_test(
                block.point.to_f32vec2() * BLOCK_SIZE,
                chunk_position,
                OUTER_SHAPE,
            )
        });
    }
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkPoint(I16Vec2::ZERO));
        app.insert_resource(UnloadBlocksMap::default());
        app.add_event::<ChunkChanged>();
        app.add_systems(Startup, start_chunk);
        app.add_systems(Update, (player_moved, without_block, with_block));
    }
    // TODO render
    // TODO spawn
    // TODO projectile
    // TODO sweep or tree?
}
