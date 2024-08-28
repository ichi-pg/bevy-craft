use crate::collision::*;
use crate::hit_test::*;
use crate::math::*;
use crate::mob_chase::*;
use crate::mob_patrol::*;
use crate::mob_stroll::*;
use crate::mob_walk::*;
use crate::player::*;
use crate::stats::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

const KNOCK_BACK_X: f32 = 400.0;
const KNOCK_BACK_Y: f32 = 1500.0;

fn spawn_enemies(mut commands: Commands) {
    let size = 128.0;
    let home_position = Vec3::new(size * 10.0, size * 10.0, 0.0);
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
            AttackSpeed(1.0),
            JumpPower(1500.0),
        ),
        (
            MobWalk,
            MobPatrol,
            MobStroll(0.0),
            HomePosition(home_position.xy()),
            HomeDistanceSquared(home_distance * home_distance),
            FindDistanceSquared((size * 5.0).pow2()),
            LostDistanceSquared((size * 5.0).pow2()),
            AttackDistanceSquared((size * 3.0).pow2()),
            PrevPosition(home_position.xy()),
        ),
    ));
    // TODO spawner
    // TODO texture animation
}

fn player_collided(
    query: Query<&AttackPower, (With<Enemy>, With<EnemyCollided>)>,
    mut event_writer: EventWriter<PlayerDamaged>,
) {
    for attack_power in &query {
        event_writer.send(PlayerDamaged(attack_power.0));
    }
    // TODO which player?
}

fn projectile_collided(
    mut query: Query<
        (Entity, &mut Health, &mut Velocity2, &Direction2),
        (With<Enemy>, With<PlayerProjectileCollided>),
    >,
    player_query: Query<&AttackPower, With<Player>>,
    mut commands: Commands,
) {
    for (entity, mut health, mut velocity, direction) in &mut query {
        for attack_power in &player_query {
            health.0 -= attack_power.0;
            if health.0 <= 0.0 {
                health.0 = 0.0;
                commands.entity(entity).despawn_recursive();
            } else {
                velocity.x = -direction.x * KNOCK_BACK_X;
                velocity.y = KNOCK_BACK_Y;
                commands.entity(entity).insert(KnockBack);
            }
            println!("{}", health.0);
        }
    }
    // TODO health gauge
    // TODO damaged and dead effect
    // TODO which player?
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
        app.add_systems(Update, (player_collided, projectile_collided));
    }
}
