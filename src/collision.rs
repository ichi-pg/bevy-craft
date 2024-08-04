use crate::grounded::*;
use crate::hit_test::*;
use crate::rigid_body::*;
use arrayvec::ArrayVec;
use bevy::prelude::*;

#[derive(Event)]
pub struct Collided;

pub fn notify_collision<T: Component, U: Component>(
    query1: Query<(&Transform, &Shape), (With<T>, Changed<Transform>)>,
    query2: Query<(Entity, &Transform, &Shape), With<U>>,
    mut commands: Commands,
) {
    for (transform2, shape2) in &query1 {
        for (entity, transform1, shape1) in &query2 {
            // Broad Phase
            if !aabb_test(
                transform1.translation,
                *shape1,
                transform2.translation,
                *shape2,
            ) {
                continue;
            }
            // Narrow Phase
            let repulsion = shape_and_shape(
                transform1.translation.xy(),
                *shape1,
                transform2.translation.xy(),
                *shape2,
            );
            if repulsion == Vec2::ZERO {
                continue;
            }
            commands.entity(entity).insert(Collided);
        }
    }
    // TODO chunk or sweep or tree
}

pub fn dynamics_collision<T: Component, U: Component>(
    mut query1: Query<
        (Entity, &mut Transform, &Shape, &mut Velocity2),
        (With<T>, Changed<Transform>),
    >,
    query2: Query<(&Transform, &Shape), (With<U>, Without<T>)>,
    mut commands: Commands,
) {
    for (entity, mut transform1, shape1, mut velocity) in &mut query1 {
        let mut hits = ArrayVec::<_, 16>::new();
        for (transform2, shape2) in &query2 {
            // Broad Phase
            if !aabb_test(
                transform1.translation,
                *shape1,
                transform2.translation,
                *shape2,
            ) {
                continue;
            }
            // Narrow Phase
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
        // Dynamics Phase
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
            commands.entity(entity).insert(Grounded);
        } else if -repulsions.y > x_abs && velocity.y > 0.0 {
            velocity.y = 0.0;
        }
    }
    // FIXME jump out
    // TODO chunk or sweep or tree
    // TODO refactor velocity, grounded, and hit head.
    // TODO dynamics gizmo
    // TODO collision profiler
}
