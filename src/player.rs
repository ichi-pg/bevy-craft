use crate::collision::*;
use crate::grounded::*;
use crate::input::*;
use crate::rigid_body::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct PlayerController;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = 128.0;
    commands.spawn((
        // SpriteBundle {
        //     sprite: Sprite {
        //         color: Color::WHITE,
        //         custom_size: Some(Vec2::new(size, size)),
        //         ..default()
        //     },
        //     ..default()
        // },
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(size * 0.5))),
            material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
            ..default()
        },
        PlayerController,
        RigitBodyController,
        Velocity2::default(),
        BroadHits::default(),
        NarrowHits::default(),
        Collider::circle(size * 0.5),
    ));
}

fn add_move_x(mut players: Query<&mut Velocity2, With<PlayerController>>, input: Res<Input>) {
    for mut velocity in &mut players {
        velocity.x = input.left_stick.x * 400.0;
    }
}

// fn add_move_xy(
//     mut players: Query<&mut Velocity2, With<PlayerController>>,
//     input: Res<Input>,
// ) {
//     for mut velocity in &mut players {
//         if input.left_stick.x != 0.0 || input.left_stick.y != 0.0 {
//             velocity.0 = input.left_stick.normalize() * 400.0;
//         } else {
//             velocity.0 = Vec2::ZERO;
//         }
//     }
// }

fn add_jump(
    mut players: Query<&mut Velocity2, (With<PlayerController>, With<Grounded>)>,
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                // add_move_xy,
                add_move_x, add_jump,
            ),
        );
    }
}
