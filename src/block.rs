use crate::atlas::*;
use crate::chunk::*;
use crate::click_shape::*;
use crate::collision::*;
use crate::hit_test::*;
use crate::hotbar::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::item_id::*;
use crate::item_node::*;
use crate::item_selecting::*;
use crate::liquid::*;
use crate::player::*;
use crate::random::*;
use crate::stats::*;
use crate::storage::*;
use crate::surface::*;
use crate::tree::*;
use crate::workbench::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;
use rand::RngCore;
use std::collections::HashMap;

pub const BLOCK_SIZE: f32 = 128.0;
pub const HALF_BLOCK_SIZE: f32 = BLOCK_SIZE * 0.5;
pub const INVERTED_BLOCK_SIZE: f32 = 1.0 / BLOCK_SIZE;

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

pub struct PlacedBlock {
    pub item_id: u16,
    pub pressure: bool,
    pub tree_power: u8,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PlacedBlockMap(pub HashMap<I16Vec2, PlacedBlock>);

pub trait BuildBlock {
    fn build_block(
        &mut self,
        item_id: u16,
        point: I16Vec2,
        attribute_map: &ItemAttributeMap,
        atlas_map: &AtlasMap,
        random: &mut Random,
    );
}

impl<'w, 's> BuildBlock for Commands<'w, 's> {
    fn build_block(
        &mut self,
        item_id: u16,
        point: I16Vec2,
        attribute_map: &ItemAttributeMap,
        atlas_map: &AtlasMap,
        random: &mut Random,
    ) {
        let Some(attribute) = attribute_map.get(&item_id) else {
            todo!()
        };
        let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
            todo!()
        };
        let bundle = (
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                    ..default()
                },
                texture: atlas.texture.clone(),
                transform: Transform::from_xyz(
                    point.x as f32 * BLOCK_SIZE,
                    point.y as f32 * BLOCK_SIZE,
                    0.0,
                ),
                ..default()
            },
            TextureAtlas {
                layout: atlas.layout.clone(),
                index: attribute.atlas_index as usize,
            },
            Shape::Rect(Vec2::new(HALF_BLOCK_SIZE, HALF_BLOCK_SIZE)),
            Block,
            BlockID(random.next_u64()),
            ItemID(item_id),
            Health(100.0),
            MaxHealth(100.0),
            InChunk,
        );
        match item_id {
            WATER_ITEM_ID => self.spawn((bundle, Liquid, Uncollide)),
            LAVA_ITEM_ID => self.spawn((bundle, Liquid, Uncollide)),
            WOOD_ITEM_ID => self.spawn((bundle, Tree, Uncollide)),
            SOIL_ITEM_ID => self.spawn((bundle, Surface)),
            _ => self.spawn(bundle),
        };
        // TODO not overlap block id
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
    mut block_map: ResMut<PlacedBlockMap>,
    time: Res<Time>,
) {
    for (entity, transform, item_id, block_id, mut health) in &mut query {
        for (pickaxe_power, attack_speed) in &player_query {
            health.0 -= pickaxe_power.0 * time.delta_seconds() * attack_speed.0;
        }
        if health.0 <= 0.0 {
            let position = transform.translation.xy();
            let point = (position * INVERTED_BLOCK_SIZE).as_i16vec2();
            commands.entity(entity).despawn_recursive();
            block_map.remove(&point);
            block_event_writer.send(BlockDestroied {
                position,
                block_id: block_id.0,
            });
            item_event_writer.send(ItemDropped {
                position,
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
    // TODO sync minimap
    // TODO can't destroy liquid
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
            STORAGE_ITEM_ID => {
                storage_event_writer.send(StorageClicked {
                    block_id: block_id.0,
                });
            }
            WORKBENCH_ITEM_ID => {
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
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
    mut query: Query<(&mut ItemID, &mut ItemAmount, &ItemIndex), With<HotbarItem>>,
    mut event_reader: EventReader<EmptyClicked>,
    mut commands: Commands,
    mut random: ResMut<Random>,
    mut block_map: ResMut<PlacedBlockMap>,
) {
    for event in event_reader.read() {
        for (mut item_id, mut amount, index) in &mut query {
            if index.0 != selected.0 {
                continue;
            }
            if item_id.0 == 0 {
                continue;
            };
            let point = ((event.pos + HALF_BLOCK_SIZE) * INVERTED_BLOCK_SIZE)
                .floor()
                .as_i16vec2();
            commands.build_block(item_id.0, point, &attribute_map, &atlas_map, &mut random);
            amount.0 -= 1;
            if amount.0 == 0 {
                item_id.0 = 0;
            }
            block_map.insert(
                point,
                PlacedBlock {
                    item_id: item_id.0,
                    pressure: false,
                    tree_power: 0,
                },
            );
        }
    }
    // FIXME overlap item
    // TODO using selected item id resource?
    // TODO sync minimap
    // TODO can't placement item
    // TODO can placement in liquid
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlacedBlockMap::default());
        app.add_event::<BlockDestroied>();
        app.add_systems(
            Update,
            (placement_block, interact_block, repair_health, sync_health),
        );
        app.add_systems(Last, destroy_block);
    }
}
