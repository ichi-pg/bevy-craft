use avian2d::{
    // math::*,
    prelude::*
};
use bevy::prelude::*;
use crate::input::*;

#[derive(Component)]
pub struct Player;

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
        // ShapeCaster::new(
        //     Collider::circle(64.0),
        //     Vector::ZERO,
        //     0.0,
        //     Dir2::NEG_Y)
        //     .with_max_time_of_impact(10.0),
        LockedAxes::ROTATION_LOCKED,
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity::default(),
        GravityScale(128.0),
        LinearVelocity::default(),
        Player,
    ));
}

fn update_player(
    mut players: Query<&mut Transform, With<Player>>,
    input: Res<Input>,
    time: Res<Time>
) {
    let mut player = players.single_mut();
    if input.stick_pressed {
        player.translation.x += input.stick.x * 512.0 * time.delta_seconds();
        // player.x += input.stick.x * 512.0 * time.delta_seconds();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, update_player);
    }
}
