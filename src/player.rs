use crate::gravity::*;
use crate::hit_test::*;
use crate::input::*;
use crate::item_stats::*;
use crate::minimap::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
pub struct PlayerDead;

#[derive(Event)]
pub struct PlayerDamaged(pub f32);

const PLAYER_SIZE: f32 = 128.0;
pub const PLAYER_HEALTH: f32 = 100.0;
pub const PLAYER_PICKAXE_POWER: f32 = 100.0;
pub const PLAYER_MOVE_SPEED: f32 = 400.0;
pub const PLAYER_JUMP_POWER: f32 = 1500.0;
const KNOCK_BACK_X: f32 = PLAYER_SIZE * 2.0;
const KNOCK_BACK_Y: f32 = 1500.0;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                    ..default()
                },
                ..default()
            },
            Player,
            PlayerController,
            Health(PLAYER_HEALTH),
            MaxHealth(PLAYER_HEALTH),
            PickaxePower(PLAYER_PICKAXE_POWER),
            AttackPower(0.0),
            MoveSpeed(PLAYER_MOVE_SPEED),
            JumpPower(PLAYER_JUMP_POWER),
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
    // TODO texture animation
}

fn player_move(
    mut query: Query<
        (&mut Velocity2, &mut Direction2, &MoveSpeed),
        (With<PlayerController>, Without<KnockBack>),
    >,
    left_stick: Res<LeftStick>,
) {
    for (mut velocity, mut direction, move_speed) in &mut query {
        if left_stick.x != 0.0 {
            direction.x = left_stick.x;
            velocity.x = direction.x * move_speed.0;
        } else {
            velocity.x = 0.0;
        }
    }
}

fn player_jump(
    mut query: Query<
        (&mut Velocity2, &JumpPower),
        (With<PlayerController>, With<Grounded>, Without<KnockBack>),
    >,
    space: Res<Space>,
) {
    for (mut velocity, jump_power) in &mut query {
        if space.pressed {
            velocity.y = jump_power.0;
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
        transform.translation = Vec3::ZERO;
        commands.entity(entity).remove::<PlayerDead>();
    }
    // TODO you dead
    // TODO respawn point
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamaged>();
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_move, player_jump));
        app.add_systems(PostUpdate, (player_damaged, player_respawn));
    }
    // TODO after or post update?
}
