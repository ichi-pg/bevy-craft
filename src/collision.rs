use crate::block::*;
use crate::grounded::*;
use crate::hit_test::*;
use crate::item::*;
use crate::player::*;
use arrayvec::ArrayVec;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct EnableNarrow;

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
pub struct BroadHit {
    pos: Vec2,
    collider: Collider,
    enable_narrow: bool,
}

#[derive(Component)]
pub struct NarrowHit {
    pos: Vec2,
    collider: Collider,
    order: f32,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct BroadHits(ArrayVec<BroadHit, 10>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct NarrowHits(ArrayVec<NarrowHit, 10>);

#[derive(Event)]
pub struct Collided {
    pub pos: Vec2,
    // TODO spawn id
}

fn check_items(
    mut query1: Query<(&Transform, &Collider, &mut BroadHits), With<PlayerController>>,
    query2: Query<(&Transform, &Collider), With<ItemID>>,
) {
    for (transform1, collider1, mut hits) in &mut query1 {
        for (transform2, collider2) in &query2 {
            broad_test(
                transform1, collider1, transform2, collider2, false, &mut hits,
            );
        }
    }
    // TODO chunk or sweep or tree
    // TODO sleeping
    // TODO want to commonalize using layer
    // TODO should not check all players?
}

fn check_blocks(
    mut query1: Query<(&Transform, &Collider, &mut BroadHits)>,
    query2: Query<(&Transform, &Collider, Option<&EnableNarrow>), With<Block>>,
) {
    for (transform1, collider1, mut hits) in &mut query1 {
        for (transform2, collider2, enable_narrow) in &query2 {
            broad_test(
                transform1,
                collider1,
                transform2,
                collider2,
                matches!(enable_narrow, Some(_)),
                &mut hits,
            );
        }
    }
    // TODO chunk or sweep or tree
    // TODO sleeping
    // TODO want to commonalize using layer
}

#[inline(always)]
fn broad_test(
    transform1: &Transform,
    collider1: &Collider,
    transform2: &Transform,
    collider2: &Collider,
    enable_narrow: bool,
    hits: &mut BroadHits,
) {
    if aabb_test(
        transform1.translation,
        collider1.scale,
        transform2.translation,
        collider2.scale,
    ) {
        hits.push(BroadHit {
            pos: transform2.translation.xy(),
            collider: collider2.clone(),
            enable_narrow,
        });
    }
}

fn narrow_phase(
    mut query: Query<(&Transform, &Collider, &mut BroadHits, &mut NarrowHits)>,
    mut event_writer: EventWriter<Collided>,
) {
    for (transform, collider, mut broad_hits, mut narrow_hits) in &mut query {
        narrow_hits.clear();
        for hit in broad_hits.iter() {
            let push = shape_and_shape(
                transform.translation.xy(),
                collider.shape,
                collider.scale,
                hit.pos,
                hit.collider.shape,
                hit.collider.scale,
            );
            if push == Vec2::ZERO {
                continue;
            }
            if hit.enable_narrow {
                narrow_hits.push(NarrowHit {
                    pos: hit.pos,
                    collider: hit.collider,
                    order: push.length_squared(),
                });
            }
            // event_writer.send(Collided { pos: hit.pos });
        }
        narrow_hits.sort_by(|a, b| a.order.partial_cmp(&b.order).unwrap());
        broad_hits.clear();
    }
    // TODO when any hits
    // TODO collide item
}

fn dynamics_phase(
    mut query: Query<(Entity, &mut Transform, &Collider, &NarrowHits)>,
    mut commands: Commands,
) {
    for (entity, mut transform, collider, narrow_hits) in &mut query {
        let mut pushed = Vec2::ZERO;
        for hit in narrow_hits.iter() {
            let push = shape_and_shape(
                transform.translation.xy(),
                collider.shape,
                collider.scale,
                hit.pos,
                hit.collider.shape,
                hit.collider.scale,
            );
            transform.translation.x += push.x;
            transform.translation.y += push.y;
            pushed += push;
        }
        if pushed.y > pushed.x.abs() {
            commands.entity(entity).insert(Grounded);
        }
    }
    // TODO when any hits
    // TODO can replace entities?
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Collided>();
        app.add_systems(
            Update,
            (check_items, check_blocks, narrow_phase, dynamics_phase),
        );
    }
}
