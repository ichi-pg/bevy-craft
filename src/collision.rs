use bevy::prelude::*;
use arrayvec::ArrayVec;

#[derive(Component)]
pub struct Grounded;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Positioned(Vec3);

#[derive(Clone, Copy)]
enum Shape {
    Circle,
    Rect,
}

#[derive(Component, Clone, Copy)]
pub struct Collider {
    shape: Shape,
    scale: Vec2,
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
pub struct Hit {
    pub translation: Vec3,
    pub collider: Collider,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct BroadHits(ArrayVec::<Hit, 10>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct NarrowHits(ArrayVec::<Hit, 10>);

fn broad_test(
    mut players: Query<(&Transform, &mut Positioned, &mut BroadHits, &Collider)>,
    blocks: Query<(&Transform, &Collider), Without<BroadHits>>,
) {
    for (player, mut positioned, mut hits, circle) in &mut players {
        hits.clear();
        if player.translation.x == positioned.x && player.translation.y == positioned.y {
            continue;
        }
        for (block, rect) in &blocks {
            if is_hit_rect_and_rect(
                player.translation.xy(),
                circle.scale,
                block.translation.xy(),
                rect.scale,
            ) {
                hits.push(Hit {
                    translation: block.translation,
                    collider: rect.clone(),
                });
            }
        }
        positioned.x = player.translation.x;
        positioned.y = player.translation.y;
    }
    // TODO chunk
}

#[inline(always)]
fn is_hit_rect_and_rect(pos1: Vec2, scale1: Vec2, pos2: Vec2, scale2: Vec2) -> bool {
    let w = scale1.x + scale2.x;
    if pos1.x < pos2.x - w {
        return false;
    }
    if pos1.x > pos2.x + w {
        return false;
    }
    let h = scale1.y + scale2.y;
    if pos1.y < pos2.y - h {
        return false;
    }
    if pos1.y > pos2.y + h {
        return false;
    }
    return true;
}

fn narrow_test(
    mut players: Query<(&Transform, &Collider, &BroadHits, &mut NarrowHits)>,
) {
    for (transform, collider, broad_hits, mut narrow_hits) in &mut players {
        narrow_hits.clear();
        for hit in broad_hits.iter() {
            if is_hit_shape_and_shape(
                transform.translation.xy(),
                collider.shape,
                collider.scale,
                hit.translation.xy(),
                hit.collider.shape,
                hit.collider.scale
            ) {
                narrow_hits.push(hit.clone());
            }
        }
    }
}

fn is_hit_shape_and_shape(
    pos1: Vec2, shape1: Shape, scale1: Vec2,
    pos2: Vec2, shape2: Shape, scale2: Vec2
) -> bool {
    match shape1 {
        Shape::Circle => {
            match shape2 {
                Shape::Circle => {
                    is_hit_circle_and_circle(
                        pos1, scale1.x, pos2, scale2.x
                    )
                }
                Shape::Rect => {
                    is_hit_circle_and_rect(
                        pos1, scale1.x, pos2, scale2
                    )
                }
            }
        }
        Shape::Rect => {
            match shape2 {
                Shape::Circle => {
                    is_hit_circle_and_rect(
                        pos2, scale2.x, pos1, scale1
                    )
                }
                Shape::Rect => {
                    is_hit_rect_and_rect(
                        pos1, scale1, pos2, scale2
                    )
                }
            }
        }
    }
}

fn is_hit_circle_and_circle(pos1: Vec2, radius1: f32, pos2: Vec2, radius2: f32) -> bool {
    (pos1).distance_squared(pos2) < (radius1 + radius2).sqrt()
}

fn is_hit_circle_and_rect(pos1: Vec2, radius: f32, pos2: Vec2, scale2: Vec2) -> bool {
    let x = pos1.x;
    let y = pos1.y;
    let x1 = pos2.x - scale2.x;
    let x2 = pos2.x + scale2.x;
    let y1 = pos2.y - scale2.y;
    let y2 = pos2.y + scale2.y;
    if x1 < x && x < x2 && y1 - radius < y && y < y2 + radius {
        return true;
    }
    if y1 < y && y < y2 && x1 - radius < x && x < x2 + radius {
        return true;
    }
    let rr = radius.sqrt();
    if (x1 - x).sqrt() + (y1 - y).sqrt() < rr {
        return true;
    }
    if (x2 - x).sqrt() + (y1 - y).sqrt() < rr {
        return true;
    }
    if (x1 - x).sqrt() + (y2 - y).sqrt() < rr {
        return true;
    }
    if (x2 - x).sqrt() + (y2 - y).sqrt() < rr {
        return true;
    }
    return false;
}

fn slide_on_ground(
    mut players: Query<(Entity, &mut Transform, &Collider, &NarrowHits)>,
    mut commands: Commands,
) {
    for (entity, mut transform, collider, narrow_hits) in &mut players {
        for hit in narrow_hits.iter() {
            transform.translation.y = collider.scale.y + hit.translation.y + hit.collider.scale.y;
        }
        if narrow_hits.is_empty() {
            commands.entity(entity).remove::<Grounded>();
        } else {
            commands.entity(entity).insert(Grounded);
        }
    }
    // TODO slide
    // TODO bottom grounded
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            broad_test,
            narrow_test,
            slide_on_ground,
        ));
    }
}
