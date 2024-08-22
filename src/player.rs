use crate::gravity::*;
use crate::hit_test::*;
use crate::input::*;
use crate::item_stats::*;
use crate::minimap::*;
use crate::velocity::*;
use bevy::prelude::*;
use bevy::sprite::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerController;

#[derive(Component, Deref, DerefMut)]
pub struct Direction2(Vec2);

const PLAYER_SIZE: f32 = 128.0;
pub const HEALTH: f32 = 100.0;
pub const PICKAXE_POWER: f32 = 100.0;
pub const MELEE_POWER: f32 = 10.0;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(PLAYER_SIZE * 0.5))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(0.0, 128.0, 0.0),
                ..default()
            },
            Player,
            PlayerController,
            Health(HEALTH),
            MaxHealth(HEALTH),
            PickaxePower(PICKAXE_POWER),
            MeleePower(MELEE_POWER),
            JumpController,
            Velocity2::default(),
            Direction2(Vec2::X),
            Shape::Circle(PLAYER_SIZE * 0.5),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 0.0, 0.0, MINIMAP_ALPHA),
                        custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                MINIMAP_LAYER,
            ));
        });
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
