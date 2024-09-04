use crate::hit_test::*;
use crate::player::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InChunk;

#[derive(Resource, Deref, DerefMut)]
struct ChunkPosition(Vec2);

#[derive(Event)]
struct ChunkChanged;

const CHUNK_SIZE: f32 = 1280.0;
const INNER_SHAPE: Shape = Shape::Rect(Vec2::splat(CHUNK_SIZE));
const OUTER_SHAPE: Shape = Shape::Rect(Vec2::splat(CHUNK_SIZE * 2.0));

fn start_chunk(
    mut chunk_position: ResMut<ChunkPosition>,
    mut event_writer: EventWriter<ChunkChanged>,
) {
    chunk_position.x = PLAYER_RESPAWN_POSITION.x;
    chunk_position.y = PLAYER_RESPAWN_POSITION.y;
    event_writer.send(ChunkChanged);
}

fn update_chunk(
    player_query: Query<&Transform, With<PlayerController>>,
    mut chunk_position: ResMut<ChunkPosition>,
    mut event_writer: EventWriter<ChunkChanged>,
) {
    for player_transform in &player_query {
        if point_test(
            player_transform.translation.xy(),
            chunk_position.0,
            INNER_SHAPE,
        ) {
            return;
        }
        chunk_position.x = player_transform.translation.x;
        chunk_position.y = player_transform.translation.y;
        event_writer.send(ChunkChanged);
    }
}

fn chunk_changed(
    query: Query<(Entity, &Transform), With<Shape>>,
    chunk_position: Res<ChunkPosition>,
    mut commands: Commands,
    mut event_reader: EventReader<ChunkChanged>,
) {
    for _ in event_reader.read() {
        for (entity, transform) in &query {
            if point_test(transform.translation.xy(), chunk_position.0, OUTER_SHAPE) {
                commands.entity(entity).insert(InChunk);
            } else {
                commands.entity(entity).remove::<InChunk>();
            }
        }
    }
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkPosition(Vec2::ZERO));
        app.add_event::<ChunkChanged>();
        app.add_systems(Startup, start_chunk);
        app.add_systems(Update, (update_chunk, chunk_changed));
    }
    // TODO render
    // TODO spawn
    // TODO projectile
    // TODO sweep or tree?
}
