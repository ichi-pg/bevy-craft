use avian2d::{
    math::*,
    prelude::*
};
use bevy::prelude::*;
use crate::input::*;
use crate::player::*;
use crate::collision::*;

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
        Player,
        Controllable,
        // FIXME bounding
        // FIXME vibrating
    ));
}

fn update_velocity(
    mut players: Query<&mut LinearVelocity, (With<Controllable>, With<Grounded>)>,
    input: Res<Input>,
    time: Res<Time>,
) {
    if input.left_stick.x == 0.0 {
        return;
    }
    for mut velocity in &mut players {
        velocity.x += input.left_stick.x * 2048.0 * time.delta_seconds();
    }
}

fn update_grounded(
    mut players: Query<(Entity, &ShapeHits), With<Player>>,
    mut commands: Commands,
) {
    for (entity, hits) in &mut players {
        if hits.is_empty() {
            commands.entity(entity).remove::<Grounded>();
        } else {
            commands.entity(entity).insert(Grounded);
        }
    }
}

fn update_damping(
    mut players: Query<&mut LinearVelocity, With<Player>>,
) {
    for mut velocity in &mut players {
        velocity.x *= 0.98;
    }
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
