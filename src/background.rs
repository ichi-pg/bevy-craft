use crate::player::*;
use crate::z_sort::*;
use bevy::prelude::*;

#[derive(Component)]
struct Background;

fn spawn_background(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.3, 0.6, 0.9),
                custom_size: Some(Vec2::new(1920.0, 1080.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
            ..default()
        },
        Background,
    ));
    // TODO multi layers
}

fn move_background(
    mut query: Query<&mut Transform, With<Background>>,
    player_query: Query<&Transform, (With<PlayerController>, Without<Background>)>,
) {
    for mut transform in &mut query {
        for player_transform in &player_query {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
    // TODO common trace system
    // TODO change texture with environment
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_background);
        app.add_systems(Update, move_background);
    }
}
