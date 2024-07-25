use bevy::prelude::*;
use crate::input::*;
use crate::collision::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Controllable;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity(Vec3);

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..default()
            },
            ..default()
        },
        Player,
        Controllable,
        Velocity::default(),
        Positioned::default(),
        Hits::default(),
    ));
}

fn move_player(
    mut players: Query<&mut Transform, With<Controllable>>,
    input: Res<Input>,
    time: Res<Time>,
) {
    for mut transform in &mut players {
        if input.left_stick.x == 0.0 {
            continue
        }
        transform.translation.x += input.left_stick.x * 512.0 * time.delta_seconds();
    }
    // TODO velocity
}

fn jump_player(
    mut players: Query<(Entity, &mut Transform), (With<Controllable>, With<Grounded>)>,
    input: Res<Input>,
    mut commands: Commands,
) {
    for (entity, mut transform) in &mut players {
        if !input.space_pressed {
            continue
        }
        transform.translation.y += 256.0;
        commands.entity(entity).remove::<Grounded>();
    }
    // TODO velocity
}

fn fall_player(
    mut players: Query<&mut Transform, (With<Player>, Without<Grounded>)>,
    time: Res<Time>,
) {
    for mut transform in &mut players {
        transform.translation.y -= 128.0 * time.delta_seconds();
    }
    // TODO velocity
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (
            move_player,
            jump_player,
            fall_player,
        ));
    }
}
