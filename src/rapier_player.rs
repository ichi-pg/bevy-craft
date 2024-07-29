use crate::input::*;
use crate::player::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        KinematicCharacterController::default(),
        Collider::ball(64.0),
        TransformBundle::default(),
        RigidBody::KinematicPositionBased,
        Velocity2::default(),
        Controllable,
        // FIXME wall grounded
    ));
}

fn add_velocity(
    mut players: Query<(&mut KinematicCharacterController, &Velocity2)>,
    time: Res<Time>,
) {
    for (mut controller, velocity) in &mut players {
        let x = velocity.x * time.delta_seconds();
        let y = velocity.y * time.delta_seconds();
        controller.translation = match controller.translation {
            Some(v) => Some(Vec2::new(v.x + x, v.y + y)),
            None => Some(Vec2::new(x, y)),
        };
    }
}

fn add_move(mut players: Query<&mut Velocity2, With<Controllable>>, input: Res<Input>) {
    for mut velocity in &mut players {
        if input.left_stick.x == 0.0 {
            velocity.x = 0.0;
        } else {
            velocity.x = input.left_stick.x * 400.0;
        }
    }
}

fn add_jump(
    mut players: Query<(&mut Velocity2, &KinematicCharacterControllerOutput), With<Controllable>>,
    input: Res<Input>,
) {
    for (mut velocity, output) in &mut players {
        if !output.grounded {
            continue;
        }
        if input.space_pressed {
            velocity.y = 1500.0;
        } else {
            velocity.y = 0.0;
        }
    }
}

fn add_gravity(
    mut players: Query<(&mut Velocity2, &KinematicCharacterControllerOutput)>,
    time: Res<Time>,
) {
    for (mut velocity, output) in &mut players {
        if output.grounded {
            return;
        }
        velocity.y = (velocity.y - 4000.0 * time.delta_seconds()).max(-2048.0);
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (add_move, add_jump, add_gravity, add_velocity));
    }
}
