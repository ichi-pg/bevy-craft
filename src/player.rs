use crate::gravity::*;
use crate::hit_test::*;
use crate::input::*;
use crate::item_stats::*;
use crate::velocity::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerController;

#[derive(Component, Deref, DerefMut)]
pub struct Direction2(Vec2);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = 128.0;
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(size * 0.5))),
            material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 128.0, 0.0),
            ..default()
        },
        Player,
        PlayerController,
        MaxHealth(100.0),
        Health(100.0),
        JumpController,
        Velocity2::default(),
        Direction2(Vec2::X),
        Shape::Circle(size * 0.5),
    ));
}

fn add_move_x(
    mut query: Query<(&mut Velocity2, &mut Direction2), With<PlayerController>>,
    left_stick: Res<LeftStick>,
) {
    for (mut velocity, mut direction) in &mut query {
        if left_stick.x != 0.0 {
            velocity.x = left_stick.x * 400.0;
            direction.x = left_stick.x;
        } else {
            velocity.x = 0.0;
        }
    }
}

fn add_move_xy(
    mut query: Query<(&mut Velocity2, &mut Direction2), With<PlayerController>>,
    left_stick: Res<LeftStick>,
) {
    for (mut velocity, mut direction) in &mut query {
        if left_stick.0 != Vec2::ZERO {
            let normal = left_stick.normalize();
            velocity.0 = normal * 400.0;
            direction.x = normal.x;
            direction.y = normal.y;
        } else {
            velocity.0 = Vec2::ZERO;
        }
    }
}

fn add_jump(
    mut query: Query<&mut Velocity2, (With<PlayerController>, With<Grounded>)>,
    space: Res<Space>,
) {
    for mut velocity in &mut query {
        if space.pressed {
            velocity.y = 1500.0;
        } else {
            velocity.y = 0.0;
        }
    }
}

pub struct PlatformerPlayerPlugin;

impl Plugin for PlatformerPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (add_move_x, add_jump));
    }
}

pub struct TopDownPlayerPlugin;

impl Plugin for TopDownPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, add_move_xy);
    }
}
