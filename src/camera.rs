use bevy::prelude::*;
use crate::player::*;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_camera(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Transform, (With<Player>, Without<Camera>)>
) {
    let mut camera = cameras.single_mut();
    let player = players.single();
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, update_camera);
    }
}
