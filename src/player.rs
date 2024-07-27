use bevy::prelude::*;
use crate::input::*;
use crate::collision::*;

#[derive(Component)]
pub struct Controllable;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity3(Vec3);

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
        Controllable,
        Velocity3::default(),
        Positioned::default(),
        BroadHits::default(),
        NarrowHits::default(),
        Collider::circle(64.0),
    ));
}

fn add_velocity(
    mut players: Query<(&mut Transform, &Velocity3)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut players {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn add_move(
    mut players: Query<&mut Velocity3, With<Controllable>>,
    input: Res<Input>,
) {
    for mut velocity in &mut players {
        if input.left_stick.x == 0.0 {
            velocity.x = 0.0;
        } else {
            velocity.x = input.left_stick.x * 400.0;
        }
    }
}

fn add_jump(
    mut players: Query<&mut Velocity3, (With<Controllable>, With<Grounded>)>,
    input: Res<Input>,
) {
    for mut velocity in &mut players {
        if input.space_pressed {
            velocity.y = 1500.0;
        } else {
            velocity.y = 0.0;
        }
    }
}

fn add_gravity(
    mut players: Query<&mut Velocity3, Without<Grounded>>,
    time: Res<Time>,
) {
    for mut velocity in &mut players {
        velocity.y = (velocity.y - 4000.0 * time.delta_seconds()).max(-2048.0);
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (
            add_move,
            add_jump,
            add_gravity,
            add_velocity,
        ));
    }
}
