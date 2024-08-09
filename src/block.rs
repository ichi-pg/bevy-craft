use crate::click_shape::*;
use crate::hit_test::*;
use crate::item::*;
use crate::item_container::ItemIndex;
use crate::item_selecting::*;
use bevy::prelude::*;

const BLOCK_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct Block;

#[derive(Event)]
pub struct BlockDestroied;

fn new_bundle(item_id: u16, x: f32, y: f32, color: Color) -> (SpriteBundle, Shape, Block, ItemID) {
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

fn spawn_blocks(mut commands: Commands) {
    for x in -10..10 {
        for y in -10..-1 {
            if x * 2 < y {
                continue;
            }
            let item_id = if (x + y) % 2 == 0 { 1 } else { 2 };
            let rgb = item_id as f32 * 0.2 + 0.1;
            commands.spawn(new_bundle(
                item_id,
                x as f32 * BLOCK_SIZE,
                y as f32 * BLOCK_SIZE,
                Color::srgb(rgb, rgb, rgb),
            ));
        }
    }
    // TODO texture
    // TODO can merge shapes? ex. first horizontal, next vertical
}

fn destroy_block(
    query: Query<(Entity, &Transform, &ItemID), (With<Block>, With<Clicked>)>,
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
    // FIXME overlap spawn
    // TODO block hp
    // TODO pickaxe
    // TODO select item
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
            let rgb = item_id.0 as f32 * 0.2 + 0.1;
            commands.spawn(new_bundle(
                item_id.0,
                ((event.pos.x + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                ((event.pos.y + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                Color::srgb(rgb, rgb, rgb),
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
        app.add_systems(Update, placement_block);
        app.add_systems(PostUpdate, destroy_block);
    }
}
