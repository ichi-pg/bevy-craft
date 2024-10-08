use bevy::math::*;
use bevy::utils::HashMap;
use std::hash::RandomState;

pub trait Repeat<T> {
    fn repeat(&self, min: T, max: T) -> T;
}

impl Repeat<i8> for i8 {
    fn repeat(&self, min: i8, max: i8) -> i8 {
        if *self > max {
            (*self - (max - min) - 1).repeat(min, max)
        } else if *self < min {
            (*self + (max - min) + 1).repeat(min, max)
        } else {
            *self
        }
    }
}

pub trait Pow2<T> {
    fn pow2(&self) -> T;
}

impl Pow2<f32> for f32 {
    fn pow2(&self) -> f32 {
        *self * *self
    }
}

impl Pow2<u32> for u32 {
    fn pow2(&self) -> u32 {
        *self * *self
    }
}

pub trait LookAt<T> {
    fn look_at(&self, x: f32) -> T;
}

impl LookAt<f32> for f32 {
    fn look_at(&self, x: f32) -> f32 {
        if *self < x {
            1.0
        } else {
            -1.0
        }
    }
}

pub trait AsI32Vec2 {
    fn as_i32vec2(&self) -> IVec2;
}

impl AsI32Vec2 for Vec3 {
    fn as_i32vec2(&self) -> IVec2 {
        IVec2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl AsI32Vec2 for Vec2 {
    fn as_i32vec2(&self) -> IVec2 {
        IVec2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl AsI32Vec2 for I16Vec2 {
    fn as_i32vec2(&self) -> IVec2 {
        IVec2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

pub trait WithZ {
    fn with_z(&self, z: f32) -> Vec3;
}

impl WithZ for Vec2 {
    fn with_z(&self, z: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }
}

pub trait GetOrInsert<K, V, S = RandomState> {
    fn get_or_insert(&mut self, key: &K) -> &mut V;
}

pub trait GetOrDefault<K, V, S = RandomState> {
    fn get_or_default(&self, key: &K) -> V;
}

impl GetOrDefault<I16Vec2, u8> for HashMap<I16Vec2, u8> {
    fn get_or_default(&self, key: &I16Vec2) -> u8 {
        if let Some(value) = self.get(key) {
            *value
        } else {
            0
        }
    }
}
