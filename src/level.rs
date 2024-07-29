use bevy::prelude::*;
use crate::collision::*;
use crate::input::*;
use crate::item::*;
use crate::hit_test::*;

#[derive(Component)]
pub struct Block;

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
                    transform: Transform::from_xyz(
                        x as f32 * size,
                        y as f32 * size,
                        0.0
                    ),
                    ..default()
                },
                Collider::rect(size * 0.5, size * 0.5),
                Block,
            ));
        }
    }
}

fn destroy_block(
    mut blocks: Query<(Entity, &Transform, &Collider), With<Block>>,
    mut commands: Commands,
    input: Res<Input>,
) {
    if !input.left_click {
        return;
    }
    for (entity, transform, collider) in &mut blocks {
        if point_and_rect(input.cursor, transform.translation, collider.scale) {
            commands.entity(entity).despawn();
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.6, 0.6, 0.6),
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    transform: transform.clone(),
                    ..default()
                },
                Collider::circle(32.0),
                ItemID(1),
                ItemAmount(1),
            ));
        }
    }
    // TODO item collision
    // TODO chunk
    // TODO event
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blocks);
        app.add_systems(Update,
            destroy_block,
        );
    }
}
