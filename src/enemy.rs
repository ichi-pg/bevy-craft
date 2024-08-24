use crate::hit_test::*;
use crate::item_stats::*;
use crate::mob::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

fn spawn_enemies(mut commands: Commands) {
    let size = 128.0;
    let home_position = Vec3::ZERO;
    let home_distance = size * 5.0;
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
        MobWalk,
        HomePosition(home_position.xy()),
        HomeDistanceSquared(home_distance * home_distance),
        Health(100.0),
        MaxHealth(100.0),
        MoveSpeed(200.0),
        AttackPower(10.0),
        JumpPower(1500.0),
        Velocity2::default(),
        Direction2(Vec2::X),
        Shape::Circle(size * 0.5),
    ));
    // TODO spawner
    // TODO texture animation
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
    }
}
