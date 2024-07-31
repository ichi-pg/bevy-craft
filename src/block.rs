use crate::collision::*;
use crate::hit_test::*;
use crate::input::*;
use bevy::prelude::*;

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
            let size = 128.0;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if (x + y) % 2 == 0 {
                            Color::srgb(0.2, 0.2, 0.2)
                        } else {
                            Color::srgb(0.4, 0.4, 0.4)
                        },
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x as f32 * size, y as f32 * size, 0.0),
                    ..default()
                },
                Collider::rect(size * 0.5, size * 0.5),
                Block,
            ));
        }
    }
    // TODO texture
}

fn destroy_block(
    mut blocks: Query<(Entity, &Transform, &Collider), With<Block>>,
    mut commands: Commands,
    input: Res<Input>,
    mut event_writer: EventWriter<BlockDestroied>,
) {
    if !input.left_click {
        return;
    }
    for (entity, transform, collider) in &mut blocks {
        if point_test(input.cursor, transform.translation, collider.scale) {
            commands.entity(entity).despawn();
            event_writer.send(BlockDestroied {
                transform: transform.clone(),
            });
        }
    }
    // TODO chunk
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockDestroied>();
        app.add_systems(Startup, spawn_blocks);
        app.add_systems(Update, destroy_block);
    }
}
