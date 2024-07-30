use crate::block::*;
use crate::grounded::*;
use crate::hit_test::*;
use crate::item::*;
use arrayvec::ArrayVec;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Collider {
    shape: Shape,
    pub scale: Vec2,
}

impl Collider {
    pub fn circle(radius: f32) -> Self {
        Collider {
            shape: Shape::Circle,
            scale: Vec2::new(radius, radius),
        }
    }
    pub fn rect(half_width: f32, half_height: f32) -> Self {
        Collider {
            shape: Shape::Rect,
            scale: Vec2::new(half_width, half_height),
        }
    }
}

#[derive(Component)]
pub struct BroadItem {
    pos: Vec2,
    collider: Collider,
    spawn_id: SpawnID,
}

#[derive(Component)]
pub struct BroadBlock {
    pos: Vec2,
    collider: Collider,
}

#[derive(Component)]
pub struct NarrowBlock {
    pos: Vec2,
    collider: Collider,
    order: f32,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct BroadItems(ArrayVec<BroadItem, 10>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct BroadBlocks(ArrayVec<BroadBlock, 10>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct NarrowBlocks(ArrayVec<NarrowBlock, 10>);

#[derive(Event)]
pub struct ItemCollided {
    pub spawn_id: SpawnID,
}

fn broad_items(
    mut query1: Query<(&Transform, &Collider, &mut BroadItems)>,
    query2: Query<(&Transform, &Collider, &SpawnID), With<ItemID>>,
) {
    for (transform1, collider1, mut hits) in &mut query1 {
        hits.clear();
        for (transform2, collider2, spawn_id) in &query2 {
            if aabb_test(
                transform1.translation,
                collider1.scale,
                transform2.translation,
                collider2.scale,
            ) {
                hits.push(BroadItem {
                    pos: transform2.translation.xy(),
                    collider: collider2.clone(),
                    spawn_id: spawn_id.clone(),
                });
            }
        }
    }
    // TODO chunk or sweep or tree
    // TODO sleeping
    // TODO commonalize using layer
}

fn broad_blocks(
    mut query1: Query<(&Transform, &Collider, &mut BroadBlocks)>,
    query2: Query<(&Transform, &Collider), With<Block>>,
) {
    for (transform1, collider1, mut hits) in &mut query1 {
        hits.clear();
        for (transform2, collider2) in &query2 {
            if aabb_test(
                transform1.translation,
                collider1.scale,
                transform2.translation,
                collider2.scale,
            ) {
                hits.push(BroadBlock {
                    pos: transform2.translation.xy(),
                    collider: collider2.clone(),
                });
            }
        }
    }
    // TODO chunk or sweep or tree
    // TODO sleeping
    // TODO commonalize using layer
}

fn narrow_items(
    mut query: Query<(&Transform, &Collider, &BroadItems)>,
    mut event_writer: EventWriter<ItemCollided>,
) {
    for (transform, collider, hits) in &mut query {
        for hit in hits.iter() {
            let repulsion = shape_and_shape(
                transform.translation.xy(),
                collider.shape,
                collider.scale,
                hit.pos,
                hit.collider.shape,
                hit.collider.scale,
            );
            if repulsion == Vec2::ZERO {
                continue;
            }
            event_writer.send(ItemCollided { spawn_id: hit.spawn_id });
        }
    }
    // TODO when any hits
    // TODO commonalize using layer
}

fn narrow_blocks(mut query: Query<(&Transform, &Collider, &BroadBlocks, &mut NarrowBlocks)>) {
    for (transform, collider, broad_hits, mut narrow_hits) in &mut query {
        narrow_hits.clear();
        for hit in broad_hits.iter() {
            let repulsion = shape_and_shape(
                transform.translation.xy(),
                collider.shape,
                collider.scale,
                hit.pos,
                hit.collider.shape,
                hit.collider.scale,
            );
            if repulsion == Vec2::ZERO {
                continue;
            }
            narrow_hits.push(NarrowBlock {
                pos: hit.pos,
                collider: hit.collider,
                order: repulsion.length_squared(),
            });
        }
        narrow_hits.sort_by(|a, b| a.order.partial_cmp(&b.order).unwrap());
    }
    // TODO when any hits
}

fn dynamics_phase(
    mut query: Query<(Entity, &mut Transform, &Collider, &NarrowBlocks)>,
    mut commands: Commands,
) {
    for (entity, mut transform, collider, narrow_hits) in &mut query {
        let mut repulsions = Vec2::ZERO;
        for hit in narrow_hits.iter() {
            let repulsion = shape_and_shape(
                transform.translation.xy(),
                collider.shape,
                collider.scale,
                hit.pos,
                hit.collider.shape,
                hit.collider.scale,
            );
            transform.translation.x += repulsion.x;
            transform.translation.y += repulsion.y;
            repulsions += repulsion;
        }
        if repulsions.y > repulsions.x.abs() {
            commands.entity(entity).insert(Grounded);
        }
    }
    // TODO when any hits
    // TODO can replace entities?
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemCollided>();
        app.add_systems(
            Update,
            (
                broad_items,
                broad_blocks,
                narrow_items,
                narrow_blocks,
                dynamics_phase,
            ),
        );
    }
}
