use crate::block::*;
use crate::enemy::*;
use crate::gravity::*;
use crate::hit_test::*;
use crate::item::*;
use crate::player::*;
use crate::velocity::*;
use arrayvec::ArrayVec;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Collided(pub Vec2);

enum Collision {
    Static,
    Dynamic,
}

fn collision<T: Component, U: Component>(
    collision: Collision,
) -> impl FnMut(
    Query<(Entity, &mut Transform, &Shape, &mut Velocity2), (With<T>, Changed<Transform>)>,
    Query<(Entity, &Transform, &Shape), (With<U>, Without<T>)>,
    Commands,
) {
    move |mut query1, query2, mut commands| {
        for (entity1, mut transform1, shape1, mut velocity) in &mut query1 {
            let mut hits = ArrayVec::<_, 16>::new();
            let mut repulsions = ArrayVec::<_, 16>::new();
            for (entity2, transform2, shape2) in &query2 {
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
                match hits.try_push((repulsion.length_squared(), transform2, shape2, repulsion)) {
                    Ok(_) => {
                        commands.entity(entity2).insert(Collided(repulsion));
                        repulsions.push(repulsion);
                        continue;
                    }
                    Err(_) => break,
                };
            }
            if hits.is_empty() {
                continue;
            }
            match collision {
                Collision::Static => {
                    commands.entity(entity1).insert(Collided(Vec2::ZERO));
                }
                Collision::Dynamic => {
                    hits.sort_by(|a, b| match a.0.partial_cmp(&b.0) {
                        Some(ordering) => ordering,
                        None => todo!(),
                    });
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
                    commands.entity(entity1).insert(Collided(repulsions));
                }
            }
        }
    }
    // FIXME jump out when placement block to player position
    // TODO chunk or sweep or tree
    // TODO headed
    // TODO dynamics gizmo
    // TODO collision profiler
}

fn clear_collided(query: Query<Entity, With<Collided>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).remove::<Collided>();
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                clear_collided,
                (
                    collision::<Player, Item>(Collision::Static),
                    collision::<Player, Block>(Collision::Dynamic),
                    collision::<Item, Block>(Collision::Dynamic),
                    collision::<Enemy, Block>(Collision::Dynamic),
                )
                    .after(clear_collided)
                    .after(add_velocity),
            ),
        );
    }
}
