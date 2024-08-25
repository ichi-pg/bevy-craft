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
