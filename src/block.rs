use crate::click_shape::*;
use crate::collision::*;
use crate::hit_test::*;
use crate::item::*;
use crate::player::*;
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

fn destroy_block(
    query: Query<(Entity, &Transform), (With<Block>, With<Clicked>)>,
    mut commands: Commands,
    mut event_writer: EventWriter<BlockDestroied>,
) {
    for (entity, transform) in &query {
        commands.entity(entity).despawn();
        event_writer.send(BlockDestroied {
            transform: *transform,
        });
    }
    // TODO block hp
    // TODO pickaxe
    // TODO select item
}

fn placement_block(mut event_reader: EventReader<EmptyClicked>, mut commands: Commands) {
    for event in event_reader.read() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.6, 0.6, 0.6),
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    ((event.pos.x + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                    ((event.pos.y + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                    0.0,
                ),
                ..default()
            },
            Shape::Rect(Vec2::new(BLOCK_SIZE * 0.5, BLOCK_SIZE * 0.5)),
            Block,
        ));
    }
    // TODO consume item
    // TODO select item
    // TODO bundle
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockDestroied>();
        app.add_systems(Startup, spawn_blocks);
        app.add_systems(
            Update,
            (
                placement_block,
                dynamics_collision::<PlayerID, Block>,
                dynamics_collision::<ItemID, Block>,
            ),
        );
        app.add_systems(PostUpdate, destroy_block);
    }
}
