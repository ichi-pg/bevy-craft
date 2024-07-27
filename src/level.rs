use bevy::prelude::*;
use crate::collision::*;

fn spawn_blocks(mut commands: Commands) {
    for x in -10..10 {
        for y in -10..0 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if (x + y) % 2 == 0 {
                            Color::srgb(0.2, 0.2, 0.2)
                        } else {
                            Color::srgb(0.4, 0.4, 0.4)
                        },
                        custom_size: Some(Vec2::new(128.0, 128.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (x * 128) as f32,
                        (y * 128) as f32,
                        -1.0
                    ),
                    ..default()
                },
                Collider::rect(64.0, 64.0),
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
