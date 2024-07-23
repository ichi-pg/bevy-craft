use avian2d::{
    math::*,
    prelude::*
};
use bevy::prelude::*;
use crate::input::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Grounded(bool);

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
        RigidBody::Dynamic,
        Collider::circle(64.0),
        ShapeCaster::new(
            Collider::circle(64.0),
            Vector::ZERO,
            0.0,
            Dir2::NEG_Y)
            .with_max_time_of_impact(10.0),
        LockedAxes::ROTATION_LOCKED,
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity::default(),
        GravityScale(128.0),
        LinearVelocity::default(),
        Grounded::default(),
        Player,
        // FIXME bounding
        // FIXME vibrating
    ));
}

fn update_grounded(
    mut players: Query<(&mut Grounded, &ShapeHits), With<Player>>,
) {
    for (mut grounded, hits) in &mut players {
        grounded.0 = !hits.is_empty();
    }
}

fn update_velocity(
    mut players: Query<(&mut LinearVelocity, &Grounded), With<Player>>,
    input: Res<Input>,
    time: Res<Time>
) {
    let (mut velocity, grounded) = players.single_mut();
    if input.stick.x == 0.0 {
        return;
    }
    if !grounded.0 {
        return;
    }
    velocity.x += input.stick.x * 2048.0 * time.delta_seconds();
}

fn update_damping(
    mut players: Query<&mut LinearVelocity, With<Player>>,
) {
    let mut velocity = players.single_mut();
    velocity.x *= 0.98;
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(FixedUpdate, (
            update_grounded,
            update_velocity,
            update_damping,
        ));
    }
}
