use crate::block::*;
use crate::grounded::*;
use crate::hit_test::*;
use crate::item::*;
use crate::player::*;
use crate::rigid_body::*;
use arrayvec::ArrayVec;
use bevy::prelude::*;

#[derive(Component)]
pub struct BroadBlock {
    pos: Vec2,
    shape: Shape,
}

#[derive(Component)]
pub struct NarrowBlock {
    pos: Vec2,
    shape: Shape,
    order: f32,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct BroadBlocks(ArrayVec<BroadBlock, 10>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct NarrowBlocks(ArrayVec<NarrowBlock, 10>);

#[derive(Component)]
pub struct Collided;

#[derive(Component)]
struct BroadCollided;

fn broad_items(
    query1: Query<(&Transform, &Shape), (With<Player>, Changed<Transform>)>,
    query2: Query<(Entity, &Transform, &Shape), With<ItemID>>,
    mut commands: Commands,
) {
    for (transform1, shape1) in &query1 {
        for (entity, transform2, shape2) in &query2 {
            if aabb_test(
                transform1.translation,
                *shape1,
                transform2.translation,
                *shape2,
            ) {
                commands.entity(entity).insert(BroadCollided);
            }
        }
    }
    // TODO commonalize using layer
}

fn narrow_items(
    query1: Query<(&Transform, &Shape), With<Player>>,
    query2: Query<(Entity, &Transform, &Shape), (With<ItemID>, With<BroadCollided>)>,
    mut commands: Commands,
) {
    for (transform1, shape1) in &query1 {
        for (entity, transform2, shape2) in &query2 {
            let repulsion = shape_and_shape(
                transform1.translation.xy(),
                *shape1,
                transform2.translation.xy(),
                *shape2,
            );
            if repulsion != Vec2::ZERO {
                commands.entity(entity).insert(Collided);
            }
            commands.entity(entity).remove::<BroadCollided>();
        }
    }
    // TODO commonalize using layer
}

fn broad_blocks(
    mut query1: Query<(&Transform, &Shape, &mut BroadBlocks), Changed<Transform>>,
    query2: Query<(&Transform, &Shape), With<Block>>,
) {
    for (transform1, shape1, mut hits) in &mut query1 {
        hits.clear();
        for (transform2, shape2) in &query2 {
            if aabb_test(
                transform1.translation,
                *shape1,
                transform2.translation,
                *shape2,
            ) {
                match hits.try_push(BroadBlock {
                    pos: transform2.translation.xy(),
                    shape: *shape2,
                }) {
                    Ok(_) => continue,
                    Err(_) => break,
                };
            }
        }
    }
    // TODO chunk or sweep or tree
    // TODO commonalize using layer
    // TODO commonalize using filter component
}

fn narrow_blocks(
    mut query: Query<(&Transform, &Shape, &BroadBlocks, &mut NarrowBlocks), Changed<BroadBlocks>>,
) {
    for (transform, shape, broad_hits, mut narrow_hits) in &mut query {
        narrow_hits.clear();
        for hit in broad_hits.iter() {
            let repulsion = shape_and_shape(transform.translation.xy(), *shape, hit.pos, hit.shape);
            if repulsion == Vec2::ZERO {
                continue;
            }
            match narrow_hits.try_push(NarrowBlock {
                pos: hit.pos,
                shape: hit.shape,
                order: repulsion.length_squared(),
            }) {
                Ok(_) => continue,
                Err(_) => break,
            };
        }
        narrow_hits.sort_by(|a: &NarrowBlock, b| a.order.partial_cmp(&b.order).unwrap());
    }
    // TODO when any hits
    // TODO commonalize using layer
    // TODO commonalize using filter component
}

fn dynamics_blocks(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut Velocity2,
            &Shape,
            &NarrowBlocks,
        ),
        Changed<NarrowBlocks>,
    >,
    mut commands: Commands,
) {
    for (entity, mut transform, mut velocity, shape, narrow_hits) in &mut query {
        let mut repulsions = Vec2::ZERO;
        for hit in narrow_hits.iter() {
            let repulsion = shape_and_shape(transform.translation.xy(), *shape, hit.pos, hit.shape);
            transform.translation.x += repulsion.x;
            transform.translation.y += repulsion.y;
            repulsions += repulsion;
        }
        let x_abs = repulsions.x.abs();
        if repulsions.y > x_abs {
            commands.entity(entity).insert(Grounded);
        } else if -repulsions.y > x_abs && velocity.y > 0.0 {
            velocity.y = 0.0;
        }
    }
    // FIXME jump out
    // TODO refactor velocity, grounded, and hit head.
    // TODO when any hits
    // TODO can replace entities?
    // TODO dynamics gizmo
    // TODO collision profiler
    // TODO commonalize using filter component
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                broad_items,
                narrow_items,
                broad_blocks,
                narrow_blocks,
                dynamics_blocks,
            ),
        );
    }
}
