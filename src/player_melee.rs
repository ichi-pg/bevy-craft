use crate::hit_test::*;
use crate::input::*;
use crate::item::*;
use crate::item_dragging::*;
use crate::item_selecting::*;
use crate::player::*;
use crate::stats::*;
use crate::ui_states::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
struct PlayerAttack(pub f32);

#[derive(Component)]
struct MeleeAxis(f32);

#[derive(Component)]
struct MeleeProjectile;

#[derive(Component)]
pub struct PlayerProjectile;

const ATTACK_INTERVAL: f32 = 0.5;
const MELEE_ROTATE: f32 = 2.0 / ATTACK_INTERVAL;
const MELEE_SIZE: f32 = 100.0;
const MELEE_OFFSET: f32 = 150.0;

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
    mut commands: Commands,
    left_click: Res<LeftClick>,
    selected: Res<SelectedItem>,
) {
    if !left_click.pressed {
        return;
    }
    for (entity, direction) in &query {
        if selected.stats.attack_power > 0.0 || selected.stats.pickaxe_power > 0.0 {
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
                                        color: item_color(selected.item_id),
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
        if selected.stats.attack_power > 0.0 {
            commands.spawn((
                Transform::default(),
                PlayerProjectile,
                Shape::Circle(MELEE_SIZE * 0.5),
            ));
        }
    }
    // TODO pure projectile
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
                    }
                    Err(_) => continue,
                }
            }
            for entity in &projectile_query {
                commands.entity(entity).despawn_recursive();
            }
            commands.entity(entity).remove::<PlayerAttack>();
        }
    }
    // TODO link projectile id
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
    // TODO link projectile id
}

pub struct PlayerMeleePlugin;

impl Plugin for PlayerMeleePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamaged>();
        app.add_systems(
            Update,
            (
                player_attack
                    .run_if(in_state(UIStates::None))
                    .run_if(in_state(ItemDragged::None)),
                rotate_melee,
                sync_projectile,
            ),
        );
        app.add_systems(Last, player_attacked);
    }
    // TODO after or post update?
}