use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum Shape {
    Circle(f32),
    Rect(Vec2),
}

#[inline(always)]
pub fn aabb_test(pos1: Vec3, shape1: Shape, pos2: Vec3, shape2: Shape) -> bool {
    let w = match shape1 {
        Shape::Circle(radius) => radius,
        Shape::Rect(half_size) => half_size.x,
    } + match shape2 {
        Shape::Circle(radius) => radius,
        Shape::Rect(half_size) => half_size.x,
    };
    if pos1.x < pos2.x - w {
        false
    } else if pos1.x > pos2.x + w {
        false
    } else {
        let h = match shape1 {
            Shape::Circle(radius) => radius,
            Shape::Rect(half_size) => half_size.y,
        } + match shape2 {
            Shape::Circle(radius) => radius,
            Shape::Rect(half_size) => half_size.y,
        };
        if pos1.y < pos2.y - h {
            false
        } else if pos1.y > pos2.y + h {
            false
        } else {
            true
        }
    }
}

#[inline(always)]
pub fn point_test(pos1: Vec2, pos2: Vec3, shape2: Shape) -> bool {
    let w = match shape2 {
        Shape::Circle(radius) => radius,
        Shape::Rect(half_size) => half_size.x,
    };
    if pos1.x < pos2.x - w {
        false
    } else if pos1.x > pos2.x + w {
        false
    } else {
        let h = match shape2 {
            Shape::Circle(radius) => radius,
            Shape::Rect(half_size) => half_size.y,
        };
        if pos1.y < pos2.y - h {
            false
        } else if pos1.y > pos2.y + h {
            false
        } else {
            true
        }
    }
}

pub fn shape_and_shape(pos1: Vec2, shape1: Shape, pos2: Vec2, shape2: Shape) -> Vec2 {
    match shape1 {
        Shape::Circle(radius1) => match shape2 {
            Shape::Circle(radius2) => circle_and_circle(pos1, radius1, pos2, radius2),
            Shape::Rect(half_size) => circle_and_rect(pos1, radius1, pos2, half_size),
        },
        Shape::Rect(half_size1) => match shape2 {
            Shape::Circle(radius2) => circle_and_rect(pos2, radius2, pos1, half_size1),
            Shape::Rect(half_size2) => rect_and_rect(pos1, half_size1, pos2, half_size2),
        },
    }
}

pub fn rect_and_rect(pos1: Vec2, half_size1: Vec2, pos2: Vec2, half_size2: Vec2) -> Vec2 {
    println!("{} {} {} {}", pos1, half_size1, pos2, half_size2);
    Vec2::ZERO
    // TODO rect and rect
}

pub fn circle_and_circle(pos1: Vec2, radius1: f32, pos2: Vec2, radius2: f32) -> Vec2 {
    let margin = radius1 + radius2;
    let v = pos1 - pos2;
    if v.length_squared() < margin * margin {
        v.normalize() * (margin - v.length())
    } else {
        Vec2::ZERO
    }
}

pub fn circle_and_rect(pos1: Vec2, radius: f32, pos2: Vec2, half_size: Vec2) -> Vec2 {
    let x = pos1.x;
    let y = pos1.y;
    let x1 = pos2.x - half_size.x;
    let x2 = pos2.x + half_size.x;
    let y1 = pos2.y - half_size.y;
    let y2 = pos2.y + half_size.y;
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
    } else {
        let rr = radius * radius;
        let a = Vec2::new(x - x1, y - y1);
        if a.length_squared() < rr {
            a.normalize() * (radius - a.length())
        } else {
            let b = Vec2::new(x - x2, y - y1);
            if b.length_squared() < rr {
                b.normalize() * (radius - b.length())
            } else {
                let c = Vec2::new(x - x1, y - y2);
                if c.length_squared() < rr {
                    c.normalize() * (radius - c.length())
                } else {
                    let d = Vec2::new(x - x2, y - y2);
                    if d.length_squared() < rr {
                        d.normalize() * (radius - d.length())
                    } else {
                        Vec2::ZERO
                    }
                }
            }
        }
    }
    // TODO dived center
    // TODO optimize &
}
