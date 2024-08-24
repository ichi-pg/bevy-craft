use crate::collision::*;
use crate::hit_test::*;
use crate::item_stats::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct HomePosition(Vec2);

#[derive(Component)]
struct HomeDistance(f32);

#[derive(Component, PartialEq, Eq)]
enum Behavior {
    Walk,
}

fn spawn_enemies(mut commands: Commands) {
    let size = 128.0;
    let home_position = Vec3::ZERO;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.0, 1.0),
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            transform: Transform::from_translation(home_position),
            ..default()
        },
        Enemy,
        Behavior::Walk,
        HomePosition(home_position.xy()),
        HomeDistance(size * 10.0),
        Health(100.0),
        MaxHealth(100.0),
        MoveSpeed(200.0),
        AttackPower(10.0),
        // JumpController,
        Velocity2::default(),
        Direction2(Vec2::X),
        Shape::Circle(size * 0.5),
    ));
    // TODO spawner
    // TODO texture animation
}

fn enemy_walk(mut query: Query<(&Behavior, &mut Velocity2, &Direction2, &MoveSpeed), With<Enemy>>) {
    for (behavior, mut velocity, direction, move_speed) in &mut query {
        if *behavior != Behavior::Walk {
            continue;
        }
        velocity.x = direction.x * move_speed.0;
    }
    // TODO find player and chase
    // TODO stay home area
    // TODO sometimes stop walk and look around
}

fn enemy_collided(mut query: Query<(&mut Direction2, &Collided), With<Enemy>>) {
    for (mut direction, collided) in &mut query {
        if collided.y < collided.x.abs() {
            direction.x = -direction.x;
        }
    }
    // TODO jump
}

pub struct PlatformerEnemyPlugin;

impl Plugin for PlatformerEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
        app.add_systems(Update, (enemy_walk, enemy_collided));
    }
    // TODO state filter component?
}
