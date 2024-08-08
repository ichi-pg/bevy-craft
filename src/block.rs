use crate::click_shape::*;
use crate::hit_test::*;
use crate::item::*;
use bevy::prelude::*;

const BLOCK_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct Block;

#[derive(Event)]
pub struct BlockDestroied;

fn spawn_blocks(mut commands: Commands) {
    for x in -10..10 {
        for y in -10..-1 {
            if x * 2 < y {
                continue;
            }
            let item_id = if (x + y) % 2 == 0 { 1 } else { 2 };
            let rgb = item_id as f32 * 0.2 + 0.1;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(rgb, rgb, rgb),
                        custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * BLOCK_SIZE,
                        y as f32 * BLOCK_SIZE,
                        0.0,
                    ),
                    ..default()
                },
                Shape::Rect(Vec2::new(BLOCK_SIZE * 0.5, BLOCK_SIZE * 0.5)),
                Block,
                ItemID(item_id),
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

fn placement_block(mut event_reader: EventReader<EmptyClicked>, mut commands: Commands) {
    for event in event_reader.read() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.7, 0.7, 0.7),
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    ((event.pos.x + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                    ((event.pos.y + BLOCK_SIZE * 0.5) / BLOCK_SIZE).floor() * BLOCK_SIZE,
                    0.0,
                ),
                ..default()
            },
            Shape::Rect(Vec2::new(BLOCK_SIZE * 0.5, BLOCK_SIZE * 0.5)),
            Block,
            ItemID(1),
        ));
    }
    // FIXME overlap item
    // TODO consume item
    // TODO select item
    // TODO bundle
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
