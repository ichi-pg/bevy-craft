use crate::player::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_camera);
        app.add_systems(Update, trace_player::<PlayerCamera>);
    }
}
