use crate::gravity::*;
use crate::hit_test::*;
use crate::hotbar::*;
use crate::input::*;
use crate::item::*;
use crate::item_dragging::*;
use crate::item_node::*;
use crate::item_selecting::*;
use crate::minimap::*;
use crate::stats::*;
use crate::ui_states::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
struct PlayerDead;

#[derive(Component)]
struct PlayerAttack(pub f32);

#[derive(Component)]
struct MeleeAxis(f32);

#[derive(Component)]
struct MeleeProjectile;

#[derive(Component)]
pub struct PlayerProjectile;

#[derive(Event)]
pub struct PlayerDamaged(pub f32);

const PLAYER_SIZE: f32 = 128.0;
pub const PLAYER_HEALTH: f32 = 100.0;
pub const PLAYER_PICKAXE_POWER: f32 = 100.0;
pub const PLAYER_MOVE_SPEED: f32 = 400.0;
pub const PLAYER_JUMP_POWER: f32 = 1500.0;
const KNOCK_BACK_X: f32 = 400.0;
const KNOCK_BACK_Y: f32 = 1500.0;
const ATTACK_INTERVAL: f32 = 0.5;
const MELEE_ROTATE: f32 = 2.0 / ATTACK_INTERVAL;
const MELEE_SIZE: f32 = 100.0;
const MELEE_OFFSET: f32 = 150.0;

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
            AttackSpeed(1.0),
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
    for (mut velocity, jump_power) in &mut query {
        if space.pressed {
            velocity.y = jump_power.0;
        }
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
        transform.translation = Vec3::ZERO;
        commands.entity(entity).remove::<PlayerDead>();
    }
    // TODO you dead
    // TODO respawn point
}

fn player_attack(
    query: Query<
        (Entity, &Direction2),
        (
            With<PlayerController>,
            Without<PlayerAttack>,
            Without<PlayerDead>,
            Without<KnockBack>,
        ),
    >,
    item_query: Query<(&ItemID, &ItemIndex), With<HotbarItem>>,
    mut commands: Commands,
    left_click: Res<LeftClick>,
    selected: Res<SelectedItem>,
) {
    if !left_click.pressed {
        return;
    }
    for (entity, direction) in &query {
        let mut item_id = 0;
        for (hotbar_item_id, index) in &item_query {
            if selected.0 == index.0 {
                item_id = hotbar_item_id.0;
            }
        }
        if item_id == 101 || item_id == 104 {
            commands
                .entity(entity)
                .insert(PlayerAttack(0.0))
                .with_children(|parent| {
                    parent
                        .spawn((SpatialBundle::default(), MeleeAxis(-direction.x)))
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: item_color(item_id),
                                        custom_size: Some(Vec2::new(MELEE_SIZE, MELEE_SIZE)),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(0.0, MELEE_OFFSET, 0.0),
                                    ..default()
                                },
                                MeleeProjectile,
                            ));
                        });
                });
        }
        if item_id == 104 {
            commands.spawn((
                Transform::default(),
                PlayerProjectile,
                Shape::Circle(MELEE_SIZE * 0.5),
            ));
        }
    }
    // TODO item hash map
    // TODO projectile
}

fn player_attacked(
    mut query: Query<(Entity, &Children, &mut PlayerAttack, &AttackSpeed)>,
    axis_query: Query<Entity, With<MeleeAxis>>,
    projectile_query: Query<Entity, With<PlayerProjectile>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, children, mut timer, attack_speed) in &mut query {
        timer.0 += time.delta_seconds() * attack_speed.0;
        if timer.0 > ATTACK_INTERVAL {
            for child in children.iter() {
                match axis_query.get(*child) {
                    Ok(entity) => {
                        commands.entity(entity).despawn_recursive();
                        for entity in &projectile_query {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                    Err(_) => continue,
                }
            }
            commands.entity(entity).remove::<PlayerAttack>();
        }
    }
    // TODO link projectile
}

fn rotate_melee(
    player_query: Query<(&Children, &AttackSpeed), With<Player>>,
    mut query: Query<(&mut Transform, &MeleeAxis)>,
    time: Res<Time>,
) {
    for (children, attack_speed) in &player_query {
        for child in children.iter() {
            match query.get_mut(*child) {
                Ok((mut transform, axis)) => {
                    transform
                        .rotate_z(MELEE_ROTATE * time.delta_seconds() * axis.0 * attack_speed.0);
                }
                Err(_) => continue,
            }
        }
    }
    // TODO cancel by any actions?
}

fn sync_projectile(
    melee_query: Query<&GlobalTransform, (With<MeleeProjectile>, Without<PlayerProjectile>)>,
    mut query: Query<&mut Transform, With<PlayerProjectile>>,
) {
    for global_transform in &melee_query {
        for mut transform in &mut query {
            let translation = global_transform.translation();
            transform.translation.x = translation.x;
            transform.translation.y = translation.y;
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamaged>();
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                player_move,
                player_jump,
                player_direction,
                player_attack
                    .run_if(in_state(UIStates::None))
                    .run_if(in_state(ItemDragged::None)),
                player_attacked,
                rotate_melee,
                sync_projectile,
            ),
        );
        app.add_systems(PostUpdate, (player_damaged, player_respawn));
    }
    // TODO after or post update?
}
