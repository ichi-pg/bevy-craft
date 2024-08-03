use crate::grounded::*;
use crate::hit_test::*;
use crate::item::*;
use crate::player::*;
use crate::rigid_body::*;
use arrayvec::ArrayVec;
use bevy::prelude::*;

#[derive(Component)]
pub struct Collided;

#[derive(Component)]
struct BroadItem;

#[derive(Component)]
struct BroadBlock;

fn broad_items(
    query1: Query<(Entity, &Transform, &Shape), (With<Player>, Changed<Transform>)>,
    query2: Query<(Entity, &Transform, &Shape), (With<ItemID>, Without<BroadItem>)>,
    mut commands: Commands,
) {
    for (entity1, transform1, shape1) in &query1 {
        let mut found = false;
        for (entity2, transform2, shape2) in &query2 {
            if aabb_test(
                transform1.translation,
                *shape1,
                transform2.translation,
                *shape2,
            ) {
                commands.entity(entity2).insert(BroadItem);
                found = true;
            }
        }
        if found {
            commands.entity(entity1).insert(BroadItem);
        }
    }
    // TODO chunk or sweep or tree
    // TODO commonalize using layer
}

fn narrow_items(
    query1: Query<(Entity, &Transform, &Shape), (With<Player>, With<BroadItem>)>,
    query2: Query<(Entity, &Transform, &Shape), (With<ItemID>, With<BroadItem>)>,
    mut commands: Commands,
) {
    for (entity1, transform1, shape1) in &query1 {
        commands.entity(entity1).remove::<BroadItem>();
        for (entity2, transform2, shape2) in &query2 {
            commands.entity(entity2).remove::<BroadItem>();
            let repulsion = shape_and_shape(
                transform1.translation.xy(),
                *shape1,
                transform2.translation.xy(),
                *shape2,
            );
            if repulsion != Vec2::ZERO {
                commands.entity(entity2).insert(Collided);
            }
        }
    }
}

fn broad_blocks(
    query1: Query<(Entity, &Transform, &Shape), (With<Velocity2>, Changed<Transform>)>,
    query2: Query<(Entity, &Transform, &Shape), (Without<Velocity2>, Without<BroadBlock>)>,
    mut commands: Commands,
) {
    for (entity1, transform1, shape1) in &query1 {
        let mut found = false;
        for (entity2, transform2, shape2) in &query2 {
            if aabb_test(
                transform1.translation,
                *shape1,
                transform2.translation,
                *shape2,
            ) {
                commands.entity(entity2).insert(BroadBlock);
                found = true;
            }
        }
        if found {
            commands.entity(entity1).insert(BroadBlock);
        }
    }
    // TODO chunk or sweep or tree
    // TODO commonalize using layer
}

fn narrow_blocks(
    mut query1: Query<
        (Entity, &mut Transform, &Shape, &mut Velocity2),
        (With<Velocity2>, With<BroadBlock>),
    >,
    query2: Query<(Entity, &Transform, &Shape), (Without<Velocity2>, With<BroadBlock>)>,
    mut commands: Commands,
) {
    for (entity1, mut transform1, shape1, mut velocity) in &mut query1 {
        commands.entity(entity1).remove::<BroadBlock>();
        let mut hits = ArrayVec::<_, 16>::new();
        for (entity2, transform2, shape2) in &query2 {
            commands.entity(entity2).remove::<BroadBlock>();
            let repulsion = shape_and_shape(
                transform1.translation.xy(),
                *shape1,
                transform2.translation.xy(),
                *shape2,
            );
            if repulsion == Vec2::ZERO {
                continue;
            }
            match hits.try_push((repulsion.length_squared(), transform2, shape2)) {
                Ok(_) => continue,
                Err(_) => break,
            };
        }
        if hits.is_empty() {
            continue;
        }
        hits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut repulsions = Vec2::ZERO;
        for hit in hits.iter() {
            let repulsion = shape_and_shape(
                transform1.translation.xy(),
                *shape1,
                hit.1.translation.xy(),
                *hit.2,
            );
            transform1.translation.x += repulsion.x;
            transform1.translation.y += repulsion.y;
            repulsions += repulsion;
        }
        let x_abs = repulsions.x.abs();
        if repulsions.y > x_abs {
            commands.entity(entity1).insert(Grounded);
        } else if -repulsions.y > x_abs && velocity.y > 0.0 {
            velocity.y = 0.0;
        }
    }
    // FIXME jump out
    // TODO refactor velocity, grounded, and hit head.
    // TODO can replace entities??
    // TODO dynamics gizmo
    // TODO collision profiler
    // TODO can filter broad pairs?
    // TODO can split systems?
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (broad_items, narrow_items, broad_blocks, narrow_blocks),
        );
    }
}
