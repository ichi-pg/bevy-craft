use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::level::*;

fn spawn_blocks(mut commands: Commands) {
    for x in -10..10 {
        for y in -10..0 {
            commands.spawn((
                Collider::cuboid(64.0, 64.0),
                TransformBundle::from_transform(Transform::from_xyz(
                    (x * 128) as f32,
                    (y * 128) as f32,
                    0.0
                )),
                RigidBody::Fixed,
                Block,
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
