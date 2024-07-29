use bevy::prelude::*;
use arrayvec::ArrayVec;
use crate::hit_test::*;

#[derive(Component)]
pub struct Grounded;

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

#[derive(Component, Clone, Copy)]
pub struct BroadHit {
    pub pos: Vec2,
    pub collider: Collider,
}

#[derive(Component, Clone, Copy)]
pub struct NarrowHit {
    pub pos: Vec2,
    pub collider: Collider,
    pub order: f32,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct BroadHits(ArrayVec::<BroadHit, 10>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct NarrowHits(ArrayVec::<NarrowHit, 10>);

fn broad_phase(
    mut players: Query<(&Transform, &mut BroadHits, &Collider)>,
    blocks: Query<(&Transform, &Collider), Without<BroadHits>>,
) {
    for (player, mut hits, circle) in &mut players {
        hits.clear();
        for (block, rect) in &blocks {
            if aabb(
                player.translation,
                circle.scale,
                block.translation,
                rect.scale,
            ) {
                hits.push(BroadHit {
                    pos: block.translation.xy(),
                    collider: rect.clone(),
                });
            }
        }
    }
    // TODO chunk or sweep or tree
    // TODO sleeping
}

fn narrow_phase(
    mut players: Query<(&Transform, &Collider, &BroadHits, &mut NarrowHits)>,
) {
    for (
        transform,
        collider,
        broad_hits,
        mut narrow_hits,
    ) in &mut players {
        narrow_hits.clear();
        for hit in broad_hits.iter() {
            let push = shape_and_shape(
                transform.translation.xy(),
                collider.shape,
                collider.scale,
                hit.pos,
                hit.collider.shape,
                hit.collider.scale
            );
            if push == Vec2::ZERO {
                continue;
            }
            narrow_hits.push(NarrowHit {
                pos: hit.pos,
                collider: hit.collider,
                order: push.length_squared(),
            });
        }
        narrow_hits.sort_by(
            |a, b|
            a.order.partial_cmp(&b.order).unwrap()
        );
    }
    // TODO when any hits
}

fn dynamics_phase(
    mut players: Query<(Entity, &mut Transform, &Collider, &NarrowHits)>,
    mut commands: Commands,
) {
    for (entity,
        mut transform,
        collider,
        narrow_hits,
    ) in &mut players {
        let mut pushed = Vec2::ZERO;
        for hit in narrow_hits.iter() {
            let push = shape_and_shape(
                transform.translation.xy(),
                collider.shape,
                collider.scale,
                hit.pos,
                hit.collider.shape,
                hit.collider.scale
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
        app.add_systems(Update, (
            broad_phase,
            narrow_phase,
            dynamics_phase,
        ));
    }
}
