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
pub struct BroadHit {
    pub pos: Vec2,
    pub collider: Collider,
}

#[derive(Component, Clone, Copy)]
pub struct NarrowHit {
    pub pos: Vec2,
    pub collider: Collider,
    pub push_squared: f32,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct BroadHits(ArrayVec::<BroadHit, 10>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct NarrowHits(ArrayVec::<NarrowHit, 10>);

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
            let w = circle.scale.x + rect.scale.x;
            if player.translation.x < block.translation.x - w {
                continue;
            }
            if player.translation.x > block.translation.x + w {
                continue;
            }
            let h = circle.scale.y + rect.scale.y;
            if player.translation.y < block.translation.y - h {
                continue;
            }
            if player.translation.y > block.translation.y + h {
                continue;
            }
            hits.push(BroadHit {
                pos: block.translation.xy(),
                collider: rect.clone(),
            });
        }
        positioned.x = player.translation.x;
        positioned.y = player.translation.y;
    }
    // TODO chunk
}

fn narrow_test(
    mut players: Query<(&Transform, &Collider, &BroadHits, &mut NarrowHits)>,
) {
    for (transform, collider, broad_hits, mut narrow_hits) in &mut players {
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
                push_squared: push.length_squared(),
            });
        }
        narrow_hits.sort_by(
            |a, b|
            a.push_squared.partial_cmp(&b.push_squared).unwrap()
        );
    }
}

fn shape_and_shape(
    pos1: Vec2, shape1: Shape, scale1: Vec2,
    pos2: Vec2, shape2: Shape, scale2: Vec2
) -> Vec2 {
    match shape1 {
        Shape::Circle => {
            match shape2 {
                Shape::Circle => {
                    circle_and_circle(
                        pos1, scale1.x, pos2, scale2.x
                    )
                }
                Shape::Rect => {
                    circle_and_rect(
                        pos1, scale1.x, pos2, scale2
                    )
                }
            }
        }
        Shape::Rect => {
            match shape2 {
                Shape::Circle => {
                    circle_and_rect(
                        pos2, scale2.x, pos1, scale1
                    )
                }
                Shape::Rect => {
                    rect_and_rect(
                        pos1, scale1, pos2, scale2
                    )
                }
            }
        }
    }
}

fn rect_and_rect(pos1: Vec2, scale1: Vec2, pos2: Vec2, scale2: Vec2) -> Vec2 {
    let w = scale1.x + scale2.x;
    let h = scale1.y + scale2.y;
    if pos1.x < pos2.x - w {
        Vec2::ZERO
    } else if pos1.x > pos2.x + w {
        Vec2::ZERO
    } else if pos1.y < pos2.y - h {
        Vec2::ZERO
    } else if pos1.y > pos2.y + h {
        Vec2::ZERO
    } else {
        Vec2::ZERO
    }
    // TODO
}

fn circle_and_circle(pos1: Vec2, radius1: f32, pos2: Vec2, radius2: f32) -> Vec2 {
    if (pos1).distance_squared(pos2) < (radius1 + radius2).sqrt() {
        pos2 - pos1
    } else {
        Vec2::ZERO
    }
}

fn circle_and_rect(pos1: Vec2, radius: f32, pos2: Vec2, scale2: Vec2) -> Vec2 {
    let x = pos1.x;
    let y = pos1.y;
    let x1 = pos2.x - scale2.x;
    let x2 = pos2.x + scale2.x;
    let y1 = pos2.y - scale2.y;
    let y2 = pos2.y + scale2.y;
    let rr = radius.sqrt();
    let a = Vec2::new(x - x1, y - y1);
    let b = Vec2::new(x - x2, y - y1);
    let c = Vec2::new(x - x1, y - y2);
    let d = Vec2::new(x - x2, y - y2);
    if x1 < x && x < x2 && y1 - radius < y && y < y2 + radius {
        if pos1.y > pos2.y {
            Vec2::new(0.0, y2 - y + radius)
        } else {
            Vec2::new(0.0, y1 - y - radius)
        }
    } else if y1 < y && y < y2 && x1 - radius < x && x < x2 + radius {
        if pos1.x > pos2.x {
            Vec2::new(x2 - x + radius, 0.0)
        } else {
            Vec2::new(x1 - x - radius, 0.0)
        }
    } else if a.length_squared() < rr {
        a.normalize() * (radius - a.length())
    } else if b.length_squared() < rr {
        b.normalize() * (radius - b.length())
    } else if c.length_squared() < rr {
        c.normalize() * (radius - c.length())
    } else if d.length_squared() < rr {
        d.normalize() * (radius - d.length())
    } else {
        Vec2::ZERO
    }
    // TODO dived center
    // TODO natural corner
}

fn slide_on_ground(
    mut players: Query<(Entity, &mut Transform, &Collider, &NarrowHits)>,
    mut commands: Commands,
) {
    for (entity, mut transform, collider, narrow_hits) in &mut players {
        let before_y = transform.translation.y;
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
        }
        if transform.translation.y > before_y {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
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
