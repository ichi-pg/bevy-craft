use crate::click_shape::*;
use crate::hit_test::*;
use crate::hotbar::*;
use crate::item::*;
use crate::item_node::*;
use crate::item_selecting::*;
use crate::minimap::*;
use crate::player::*;
use crate::random::*;
use crate::stats::*;
use crate::storage::*;
use crate::workbench::*;
use bevy::prelude::*;
use rand::RngCore;

pub const BLOCK_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct BlockID(pub u64);

#[derive(Component)]
struct Damaged;

#[derive(Event)]
pub struct BlockDestroied {
    pub translation: Vec3,
    pub block_id: u64,
}

const REPAIR_POWER: f32 = 10.0;

fn block_bundle(
    item_id: u16,
    x: f32,
    y: f32,
    color: Color,
    block_id: u64,
) -> (
    SpriteBundle,
    Shape,
    Block,
    BlockID,
    ItemID,
    Health,
    MaxHealth,
) {
    (
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        Shape::Rect(Vec2::new(BLOCK_SIZE * 0.5, BLOCK_SIZE * 0.5)),
        Block,
        BlockID(block_id),
        ItemID(item_id),
        Health(100.0),
        MaxHealth(100.0),
    )
    // TODO not overlap block id
}

fn spawn_blocks(mut commands: Commands, mut random: ResMut<Random>) {
    for x in -19..20 {
        for y in -9..10 {
            if if x >= 0 { x } else { -x } <= y * 2 + 1 {
                continue;
            }
            let item_id = (random.next_u32() % 6) as u16 + 1;
            let color = item_color(item_id);
            commands
                .spawn(block_bundle(
                    item_id,
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    color,
                    random.next_u64(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: color.with_alpha(MINIMAP_ALPHA),
                                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                ..default()
                            },
                            ..default()
                        },
                        MINIMAP_LAYER,
                    ));
                });
        }
    }
    // TODO texture
    // TODO can merge shapes? ex. first horizontal, next vertical
}

fn destroy_block(
    mut query: Query<
        (Entity, &Transform, &ItemID, &BlockID, &mut Health),
        (With<Block>, With<LeftClicked>),
    >,
    player_query: Query<(&PickaxePower, &AttackSpeed), With<PlayerController>>,
    mut commands: Commands,
    mut item_event_writer: EventWriter<ItemDropped>,
    mut block_event_writer: EventWriter<BlockDestroied>,
    time: Res<Time>,
) {
    for (entity, transform, item_id, block_id, mut health) in &mut query {
        for (pickaxe_power, attack_speed) in &player_query {
            health.0 -= pickaxe_power.0 * time.delta_seconds() * attack_speed.0;
        }
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
            block_event_writer.send(BlockDestroied {
                translation: transform.translation,
                block_id: block_id.0,
            });
            item_event_writer.send(ItemDropped {
                translation: transform.translation,
                item_id: item_id.0,
                amount: 1,
            });
        } else {
            commands
                .entity(entity)
                .insert(Damaged)
                .remove::<LeftClicked>();
        }
    }
    // TODO pickaxe category
}

fn repair_health(
    mut query: Query<(Entity, &mut Health, &MaxHealth), (With<Block>, With<Damaged>)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut health, max_health) in &mut query {
        health.0 += REPAIR_POWER * time.delta_seconds();
        if health.0 >= max_health.0 {
            health.0 = max_health.0;
            commands.entity(entity).remove::<Damaged>();
        }
    }
}

fn sync_health(
    mut query: Query<(&Health, &MaxHealth, &mut Sprite), (With<Block>, Changed<Health>)>,
) {
    for (health, max_health, mut sprite) in &mut query {
        sprite.color.set_alpha(health.0 / max_health.0);
    }
    // TODO texture
}

fn interact_block(
    query: Query<(Entity, &ItemID, &BlockID), (With<Block>, With<RightClicked>)>,
    mut commands: Commands,
    mut storage_event_writer: EventWriter<StorageClicked>,
    mut workbench_event_writer: EventWriter<WorkbenchClicked>,
) {
    for (entity, item_id, block_id) in &query {
        match item_id.0 {
            102 => {
                storage_event_writer.send(StorageClicked {
                    block_id: block_id.0,
                });
            }
            103 => {
                workbench_event_writer.send(WorkbenchClicked);
            }
            _ => {}
        };
        commands.entity(entity).remove::<RightClicked>();
    }
}

fn placement_block(
    selected: Res<SelectedItem>,
    mut query: Query<(&mut ItemID, &mut ItemAmount, &ItemIndex), With<HotbarItem>>,
    mut event_reader: EventReader<EmptyClicked>,
    mut commands: Commands,
    mut random: ResMut<Random>,
) {
    for event in event_reader.read() {
        for (mut item_id, mut amount, index) in &mut query {
            if index.0 != selected.0 {
                continue;
            }
            if item_id.0 == 0 {
                continue;
            }
            commands.spawn(block_bundle(
                item_id.0,
                ((event.pos.x + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                ((event.pos.y + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                item_color(item_id.0),
                random.next_u64(),
            ));
            amount.0 -= 1;
            if amount.0 == 0 {
                item_id.0 = 0;
            }
        }
    }
    // FIXME overlap item
    // TODO using selected item id resource?
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockDestroied>();
        app.add_systems(Startup, spawn_blocks);
        app.add_systems(
            Update,
            (placement_block, interact_block, repair_health, sync_health),
        );
        app.add_systems(Last, destroy_block);
    }
}
