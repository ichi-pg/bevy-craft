use crate::hit_test::*;
use crate::item_stats::*;
use crate::math::*;
use crate::mob_chase::*;
use crate::mob_jump_attack::*;
use crate::mob_patrol::*;
use crate::mob_stroll::*;
use crate::mob_walk::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

fn spawn_enemies(mut commands: Commands) {
    let size = 128.0;
    let home_position = Vec3::ZERO;
    let home_distance = size * 6.0;
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
        (
            Enemy,
            Velocity2::default(),
            Direction2(Vec2::X),
            Shape::Circle(size * 0.5),
        ),
        (
            Health(100.0),
            MaxHealth(100.0),
            MoveSpeed(200.0),
            AttackPower(10.0),
            AttackSpeed(512.0),
            AttackDelay(1.0),
            AttackCoolDown(1.0),
            JumpPower(1500.0),
        ),
        (
            MobWalk,
            MobPatrol,
            MobStroll,
            HomePosition(home_position.xy()),
            HomeDistanceSquared(home_distance * home_distance),
            FindDistanceSquared((size * 5.0).pow2()),
            LostDistanceSquared((size * 5.0).pow2()),
            AttackDistanceSquared((size * 3.0).pow2()),
            PrevPosition(home_position.xy()),
            StackTimer(0.0),
            AttackTimer(0.0),
        ),
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
