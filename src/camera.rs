use crate::player::*;
use bevy::prelude::*;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn move_camera(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Transform, (With<PlayerController>, Without<Camera>)>,
) {
    for mut camera in &mut cameras {
        for player in &players {
            camera.translation.x = player.translation.x;
            camera.translation.y = player.translation.y;
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
