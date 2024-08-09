use crate::chest::*;
use crate::click_shape::*;
use crate::hit_test::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_container::*;
use crate::item_selecting::*;
use crate::random::*;
use bevy::prelude::*;
use rand::RngCore;

const BLOCK_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct Block;

#[derive(Event)]
pub struct BlockDestroied;

fn block_bundle(
    item_id: u16,
    x: f32,
    y: f32,
    color: Color,
) -> (SpriteBundle, Shape, Block, ItemID) {
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
        ItemID(item_id),
    )
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
            ));
        }
    }
    // TODO texture
    // TODO can merge shapes? ex. first horizontal, next vertical
}

fn destroy_block(
    query: Query<(Entity, &Transform, &ItemID), (With<Block>, With<LeftClicked>)>,
    mut commands: Commands,
    mut item_event_writer: EventWriter<ItemDropped>,
    mut block_event_writer: EventWriter<BlockDestroied>,
) {
    for (entity, transform, item_id) in &query {
        commands.entity(entity).despawn();
        block_event_writer.send(BlockDestroied);
        item_event_writer.send(ItemDropped {
            translation: transform.translation,
            item_id: item_id.0,
            amount: 1,
        });
    }
    // TODO block hp
    // TODO pickaxe
    // TODO select item
}

fn interact_block(
    query: Query<(Entity, &ItemID), (With<Block>, With<RightClicked>)>,
    mut chest_query: Query<&mut Visibility, Or<(With<Inventory>, With<Chest>)>>,
    mut commands: Commands,
) {
    for (entity, item_id) in &query {
        if item_id.0 == 20 {
            for mut visibility in &mut chest_query {
                *visibility = Visibility::Inherited;
            }
        }
        commands.entity(entity).remove::<RightClicked>();
    }
}

fn placement_block(
    selected: Res<SelectedItem>,
    mut query: Query<(&mut ItemID, &mut ItemAmount, &ItemIndex)>,
    mut event_reader: EventReader<EmptyClicked>,
    mut commands: Commands,
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
        app.add_systems(PostUpdate, destroy_block);
    }
}
