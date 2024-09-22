use crate::block::*;
use crate::gravity::*;
use crate::hit_test::*;
use crate::input::*;
use crate::stats::*;
use crate::velocity::*;
use crate::world_generator::*;
use crate::z_sort::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
pub struct PlayerDead;

#[derive(Event)]
pub struct PlayerDamaged(pub f32);

pub const PLAYER_HEALTH: f32 = 100.0;
pub const PLAYER_PICKAXE_POWER: f32 = 100.0;
pub const PLAYER_ATTACK_POWER: f32 = 0.0;
pub const PLAYER_ATTACK_SPEED: f32 = 1.0;
pub const PLAYER_MOVE_SPEED: f32 = 400.0;
pub const PLAYER_JUMP_POWER: f32 = 1500.0;
pub const PLAYER_RESPAWN_POSITION: Vec3 =
    Vec3::new(0.0, BLOCK_SIZE * SURFACE_HEIGHT as f32 * 0.2, CHARACTER_Z);

const PLAYER_SIZE: f32 = 128.0;
const KNOCK_BACK_X: f32 = 400.0;
const KNOCK_BACK_Y: f32 = 1500.0;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.5, 0.5),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(PLAYER_RESPAWN_POSITION),
            ..default()
        },
        Player,
        PlayerController,
        Health(PLAYER_HEALTH),
        MaxHealth(PLAYER_HEALTH),
        PickaxePower(PLAYER_PICKAXE_POWER),
        AttackPower(PLAYER_ATTACK_POWER),
        AttackSpeed(PLAYER_ATTACK_SPEED),
        MoveSpeed(PLAYER_MOVE_SPEED),
        JumpPower(PLAYER_JUMP_POWER),
        Velocity2::default(),
        Direction2(Vec2::X),
        Shape::Circle(PLAYER_SIZE * 0.5),
    ));
    // TODO texture animation
}

fn player_move(
    mut query: Query<
        (&mut Velocity2, &MoveSpeed),
        (
            With<PlayerController>,
            Without<PlayerDead>,
            Without<KnockBack>,
        ),
    >,
    left_stick: Res<LeftStick>,
) {
    for (mut velocity, move_speed) in &mut query {
        if left_stick.x != 0.0 {
            velocity.x = left_stick.x * move_speed.0;
        } else {
            velocity.x = 0.0;
        }
    }
}

fn player_jump(
    mut query: Query<
        (&mut Velocity2, &JumpPower),
        (
            With<PlayerController>,
            With<Grounded>,
            Without<PlayerDead>,
            Without<KnockBack>,
        ),
    >,
    space: Res<Space>,
) {
    if !space.pressed {
        return;
    }
    for (mut velocity, jump_power) in &mut query {
        velocity.y = jump_power.0;
    }
}

fn player_direction(
    mut query: Query<(&mut Direction2, &Transform), With<PlayerController>>,
    cursor: Res<WorldCursor>,
) {
    for (mut direction, transform) in &mut query {
        direction.x = if cursor.position.x > transform.translation.x {
            1.0
        } else {
            -1.0
        }
    }
}

fn player_damaged(
    mut query: Query<
        (Entity, &mut Health, &mut Velocity2, &Direction2),
        (With<Player>, Without<PlayerDead>, Without<KnockBack>),
    >,
    mut event_reader: EventReader<PlayerDamaged>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for (entity, mut health, mut velocity, direction) in &mut query {
            health.0 -= event.0;
            if health.0 <= 0.0 {
                health.0 = 0.0;
                commands.entity(entity).insert(PlayerDead);
            } else {
                velocity.x = -direction.x * KNOCK_BACK_X;
                velocity.y = KNOCK_BACK_Y;
                commands.entity(entity).insert(KnockBack);
            }
        }
    }
    // TODO knock down or invincibility time?
    // TODO damaged and dead effect
    // TODO lost items
}

fn player_respawn(
    mut query: Query<(Entity, &mut Transform, &mut Health, &MaxHealth), With<PlayerDead>>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut health, max_health) in &mut query {
        health.0 = max_health.0;
        transform.translation = PLAYER_RESPAWN_POSITION;
        commands.entity(entity).remove::<PlayerDead>();
    }
    // TODO you dead
    // TODO respawn point
}

pub fn trace_player<T: Component>(
    scale: f32,
) -> impl FnMut(
    Query<&mut Transform, With<T>>,
    Query<&Transform, (With<PlayerController>, Without<T>, Changed<Transform>)>,
) {
    move |mut query, player_query| {
        for player_transform in &player_query {
            for mut transform in &mut query {
                transform.translation.x = player_transform.translation.x * scale;
                transform.translation.y = player_transform.translation.y * scale;
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamaged>();
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_move, player_jump, player_direction));
        app.add_systems(PostUpdate, (player_damaged, player_respawn));
    }
    // TODO after or post update?
}
