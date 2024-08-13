use crate::chest::*;
use crate::click_shape::*;
use crate::hit_test::*;
use crate::hotbar::*;
use crate::item::*;
use crate::item_container::*;
use crate::item_selecting::*;
use crate::random::*;
use bevy::prelude::*;
use rand::RngCore;

const BLOCK_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct BlockID(pub u64);

#[derive(Event)]
pub struct BlockDestroied {
    pub translation: Vec3,
    pub block_id: u64,
}

fn block_bundle(
    item_id: u16,
    x: f32,
    y: f32,
    color: Color,
    block_id: u64,
) -> (SpriteBundle, Shape, Block, BlockID, ItemID) {
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
    )
    // TODO not overlap block id
}

fn spawn_blocks(mut commands: Commands, mut random: ResMut<Random>) {
    for x in -9..10 {
        for y in -9..10 {
            if if x >= 0 { x } else { -x } <= y * 2 + 1 {
                continue;
            }
            let item_id = (random.next_u32() % 20) as u16 + 1;
            commands.spawn(block_bundle(
                item_id,
                x as f32 * BLOCK_SIZE,
                y as f32 * BLOCK_SIZE,
                item_color(item_id),
                random.next_u64(),
            ));
        }
    }
    // TODO texture
    // TODO can merge shapes? ex. first horizontal, next vertical
}

fn destroy_block(
    query: Query<(Entity, &Transform, &ItemID, &BlockID), (With<Block>, With<LeftClicked>)>,
    mut commands: Commands,
    mut item_event_writer: EventWriter<ItemDropped>,
    mut block_event_writer: EventWriter<BlockDestroied>,
) {
    for (entity, transform, item_id, block_id) in &query {
        commands.entity(entity).despawn();
        block_event_writer.send(BlockDestroied {
            translation: transform.translation,
            block_id: block_id.0,
        });
        item_event_writer.send(ItemDropped {
            translation: transform.translation,
            item_id: item_id.0,
            amount: 1,
        });
    }
    // TODO block hp
    // TODO pickaxe
}

fn interact_block(
    query: Query<(Entity, &ItemID, &BlockID), (With<Block>, With<RightClicked>)>,
    mut commands: Commands,
    mut chest_event_writer: EventWriter<ChestClicked>,
) {
    for (entity, item_id, block_id) in &query {
        match item_id.0 {
            1 => {
                chest_event_writer.send(ChestClicked {
                    block_id: block_id.0,
                });
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
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockDestroied>();
        app.add_systems(Startup, spawn_blocks);
        app.add_systems(Update, (placement_block, interact_block));
        app.add_systems(Last, destroy_block);
    }
}
