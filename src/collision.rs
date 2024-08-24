use crate::block::*;
use crate::enemy::*;
use crate::gravity::*;
use crate::hit_test::*;
use crate::item::*;
use crate::player::*;
use crate::profiler::CollisionCounter;
use crate::velocity::*;
use arrayvec::ArrayVec;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component, Default, Collided)]
pub struct ItemCollided {
    pub repulsion: Vec2,
}

#[derive(Component, Default, Collided)]
pub struct BlockCollided {
    pub repulsion: Vec2,
}

trait Collided {
    fn set_repulsion(&mut self, repulsion: Vec2);
}

enum Collision {
    Static,
    Dynamic,
}

fn collision<T: Component, U: Component, V: Component + Default + Collided>(
    collision: Collision,
) -> impl FnMut(
    Query<(Entity, &mut Transform, &Shape, &mut Velocity2), (With<T>, Changed<Transform>)>,
    Query<(Entity, &Transform, &Shape), (With<U>, Without<T>)>,
    Commands,
    ResMut<CollisionCounter>,
) {
    move |mut query1, query2, mut commands, mut counter| {
        for (entity1, mut transform1, shape1, mut velocity) in &mut query1 {
            let mut hits = ArrayVec::<_, 16>::new();
            let mut repulsions = ArrayVec::<_, 16>::new();
            for (entity2, transform2, shape2) in &query2 {
                #[cfg(any(debug_assertions, target_arch = "wasm32"))]
                {
                    counter.0 += 1;
                }
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
                    Ok(_) => {
                        let mut collided = V::default();
                        collided.set_repulsion(repulsion);
                        commands.entity(entity2).insert(collided);
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
                    commands.entity(entity1).insert(V::default());
                }
                Collision::Dynamic => {
                    hits.sort_by(|a, b| match a.0.partial_cmp(&b.0) {
                        Some(ordering) => ordering,
                        None => todo!(),
                    });
                    let mut repulsions = Vec2::ZERO;
                    for hit in hits.iter() {
                        // Merge repulsions
                        let repulsion = shape_and_shape(
                            transform1.translation.xy(),
                            *shape1,
                            hit.1.translation.xy(),
                            *hit.2,
                        );
                        transform1.translation.x += repulsion.x;
                        transform1.translation.y += repulsion.y;
                        repulsions += repulsion;
                        // Check grounded
                        let x_abs = repulsion.x.abs();
                        if repulsion.y > x_abs {
                            velocity.y = 0.0;
                            commands.entity(entity1).insert(Grounded);
                        } else if -repulsion.y > x_abs && velocity.y > 0.0 {
                            velocity.y = 0.0;
                        }
                    }
                    let mut collided = V::default();
                    collided.set_repulsion(repulsions);
                    commands.entity(entity1).insert(collided);
                }
            }
        }
    }
    // FIXME jump out when placement block to player position
    // TODO chunk or sweep or tree
    // TODO dynamics gizmo
    // TODO optimize grounded and heading
}

fn clear_collided<T: Component + Collided>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).remove::<T>();
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                clear_collided::<ItemCollided>,
                clear_collided::<BlockCollided>,
                (
                    collision::<Player, Item, ItemCollided>(Collision::Static),
                    collision::<Player, Block, BlockCollided>(Collision::Dynamic),
                    collision::<Item, Block, BlockCollided>(Collision::Dynamic),
                    collision::<Enemy, Block, BlockCollided>(Collision::Dynamic),
                )
                    .after(clear_collided::<ItemCollided>)
                    .after(clear_collided::<BlockCollided>)
                    .after(add_velocity),
            ),
        );
    }
}
