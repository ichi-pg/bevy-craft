use crate::hit_test::*;
use crate::input::*;
use bevy::prelude::*;

const BLOCK_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct Block;

#[derive(Event)]
pub struct BlockDestroied {
    pub transform: Transform,
}

fn spawn_blocks(mut commands: Commands) {
    for x in -10..10 {
        for y in -10..-1 {
            if x * 2 < y {
                continue;
            }
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if (x + y) % 2 == 0 {
                            Color::srgb(0.2, 0.2, 0.2)
                        } else {
                            Color::srgb(0.4, 0.4, 0.4)
                        },
                        custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        0.0,
                    ),
                    ..default()
                },
                Shape::Rect(Vec2::new(BLOCK_SIZE * 0.5, BLOCK_SIZE * 0.5)),
                Block,
            ));
        }
    }
    // TODO texture
}

fn touch_block(
    mut block_query: Query<(Entity, &Transform, &Shape), With<Block>>,
    other_query: Query<(&Transform, &Shape), Without<Block>>,
    mut commands: Commands,
    input: Res<Input>,
    mut event_writer: EventWriter<BlockDestroied>,
) {
    if !input.left_click {
        return;
    }
    for (entity, transform, shape) in &mut block_query {
        if point_test(input.cursor, transform.translation, *shape) {
            commands.entity(entity).despawn();
            event_writer.send(BlockDestroied {
                transform: *transform,
            });
            return;
        }
    }
    for (transform, shape) in &other_query {
        if point_test(input.cursor, transform.translation, *shape) {
            return;
        }
    }
    let x = ((input.cursor.x + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE;
    let y = ((input.cursor.y + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.6, 0.6, 0.6),
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        Shape::Rect(Vec2::new(BLOCK_SIZE * 0.5, BLOCK_SIZE * 0.5)),
        Block,
    ));
    // TODO chunk
    // TODO bundle
    // TODO clicked event
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockDestroied>();
        app.add_systems(Startup, spawn_blocks);
        app.add_systems(Update, touch_block);
    }
}
