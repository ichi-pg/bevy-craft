use crate::player::*;
use bevy::prelude::*;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn move_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<PlayerController>, Without<Camera>)>,
) {
    for mut transform in &mut query {
        for player_transform in &player_query {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, move_camera);
    }
}
