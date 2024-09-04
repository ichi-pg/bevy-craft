use crate::atlas::*;
use crate::chunk::*;
use crate::click_shape::*;
use crate::hit_test::*;
use crate::hotbar::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::item_id::*;
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

const BLOCK_SIZE: f32 = 128.0;
const REPAIR_POWER: f32 = 10.0;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct BlockID(pub u64);

#[derive(Component)]
struct Damaged;

#[derive(Event)]
pub struct BlockDestroied {
    pub position: Vec2,
    pub block_id: u64,
}

fn block_bundle(
    item_id: u16,
    x: f32,
    y: f32,
    block_id: u64,
    attribute: &ItemAttribute,
    atlas: &Atlas,
) -> (
    SpriteBundle,
    TextureAtlas,
    Shape,
    Block,
    BlockID,
    ItemID,
    Health,
    MaxHealth,
    InChunk,
) {
    (
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                ..default()
            },
            texture: atlas.texture.clone(),
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        TextureAtlas {
            layout: atlas.layout.clone(),
            index: attribute.atlas_index as usize,
        },
        Shape::Rect(Vec2::new(BLOCK_SIZE * 0.5, BLOCK_SIZE * 0.5)),
        Block,
        BlockID(block_id),
        ItemID(item_id),
        Health(100.0),
        MaxHealth(100.0),
        InChunk,
    )
    // TODO not overlap block id
    // TODO plant growth

    // TODO tree
    // TODO flower
    // TODO soil
    // TODO stone
    // TODO water
    // TODO torch

    // TODO forge
    // TODO enchant table
    // TODO door
    // TODO ladder (rope)
    // TODO scaffold
    // TODO steps

    // TODO rail
    // TODO trolley
    // TODO warp gate
    // TODO belt conveyor
    // TODO mechanical arm
    // TODO assembly machine
}

fn spawn_blocks(
    mut commands: Commands,
    mut random: ResMut<Random>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
) {
    let blocks = [GRASS_ID, WOOD_ID, STONE_ID, SOIL_ID];
    for x in -19..20 {
        for y in -9..10 {
            if if x >= 0 { x } else { -x } <= y * 2 + 1 {
                continue;
            }
            let item_id = blocks[random.next_u32() as usize % blocks.len()];
            let Some(attribute) = attribute_map.get(&item_id) else {
                return;
            };
            let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
                return;
            };
            commands
                .spawn(block_bundle(
                    item_id,
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    random.next_u64(),
                    attribute,
                    atlas,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::WHITE.with_alpha(MINIMAP_ALPHA),
                                custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                                ..default()
                            },
                            texture: atlas.texture.clone(),
                            ..default()
                        },
                        TextureAtlas {
                            layout: atlas.layout.clone(),
                            index: attribute.atlas_index as usize,
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
                position: transform.translation.xy(),
                block_id: block_id.0,
            });
            item_event_writer.send(ItemDropped {
                position: transform.translation.xy(),
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
    // TODO resource
}

fn placement_block(
    selected: Res<SelectedIndex>,
    mut query: Query<(&mut ItemID, &mut ItemAmount, &ItemIndex), With<HotbarItem>>,
    mut event_reader: EventReader<EmptyClicked>,
    mut commands: Commands,
    mut random: ResMut<Random>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
) {
    for event in event_reader.read() {
        for (mut item_id, mut amount, index) in &mut query {
            if index.0 != selected.0 {
                continue;
            }
            if item_id.0 == 0 {
                continue;
            }
            let Some(attribute) = attribute_map.get(&item_id.0) else {
                return;
            };
            let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
                return;
            };
            commands.spawn(block_bundle(
                item_id.0,
                ((event.pos.x + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                ((event.pos.y + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                random.next_u64(),
                attribute,
                atlas,
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
