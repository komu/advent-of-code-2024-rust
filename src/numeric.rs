use std::ops::{Mul, Sub};
use crate::vec2::Vec2;

pub fn count_digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        n.ilog10() + 1
    }
}

pub fn det<T : Mul<T, Output = T> + Sub<T, Output = T>>(v: Vec2<T>, u: Vec2<T>) -> T {
    v.x * u.y - v.y * u.x
}


pub fn is_integer(x: f64) -> bool {
    x.fract() == 0.0
}
