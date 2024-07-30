use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Shape {
    Circle,
    Rect,
}

#[inline(always)]
pub fn aabb_test(pos1: Vec3, scale1: Vec2, pos2: Vec3, scale2: Vec2) -> bool {
    let w = scale1.x + scale2.x;
    if pos1.x < pos2.x - w {
        false
    } else if pos1.x > pos2.x + w {
        false
    } else {
        let h = scale1.y + scale2.y;
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
pub fn point_test(pos1: Vec2, pos2: Vec3, scale2: Vec2) -> bool {
    if pos1.x < pos2.x - scale2.x {
        false
    } else if pos1.x > pos2.x + scale2.x {
        false
    } else if pos1.y < pos2.y - scale2.y {
        false
    } else if pos1.y > pos2.y + scale2.y {
        false
    } else {
        true
    }
}

pub fn shape_and_shape(
    pos1: Vec2,
    shape1: Shape,
    scale1: Vec2,
    pos2: Vec2,
    shape2: Shape,
    scale2: Vec2,
) -> Vec2 {
    match shape1 {
        Shape::Circle => match shape2 {
            Shape::Circle => circle_and_circle(pos1, scale1.x, pos2, scale2.x),
            Shape::Rect => circle_and_rect(pos1, scale1.x, pos2, scale2),
        },
        Shape::Rect => match shape2 {
            Shape::Circle => circle_and_rect(pos2, scale2.x, pos1, scale1),
            Shape::Rect => rect_and_rect(pos1, scale1, pos2, scale2),
        },
    }
}

pub fn rect_and_rect(pos1: Vec2, scale1: Vec2, pos2: Vec2, scale2: Vec2) -> Vec2 {
    println!("{} {} {} {}", pos1, scale1, pos2, scale2);
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

pub fn circle_and_rect(pos1: Vec2, radius: f32, pos2: Vec2, scale2: Vec2) -> Vec2 {
    let x = pos1.x;
    let y = pos1.y;
    let x1 = pos2.x - scale2.x;
    let x2 = pos2.x + scale2.x;
    let y1 = pos2.y - scale2.y;
    let y2 = pos2.y + scale2.y;
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
        let b = Vec2::new(x - x2, y - y1);
        let c = Vec2::new(x - x1, y - y2);
        let d = Vec2::new(x - x2, y - y2);
        if a.length_squared() < rr {
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
    }
    // TODO dived center
}
