use avian2d::prelude::*;
use bevy::prelude::*;
mod camera;
mod player;
mod avian_player;
mod input;
mod level;
mod avian_level;
mod collision;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Craft".into(),
                    resolution: (1920.0, 1080.0).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            input::InputPlugin,
            collision::CollisionPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            level::LevelPlugin,
        ))
        .run();
}
