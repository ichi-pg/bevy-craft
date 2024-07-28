use bevy::prelude::*;
use crate::collision::*;

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
                        -1.0
                    ),
                    ..default()
                },
                Collider::rect(size * 0.5, size * 0.5),
            ));
        }
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blocks);
    }
}
