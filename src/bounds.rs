use crate::vec2::Vec2;
use num::Zero;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Bounds<T> {
    w: T,
    h: T,
}

impl<T> Bounds<T> {
    pub fn new(w: T, h: T) -> Self {
        Bounds { w, h }
    }
}

impl<T: Ord + Zero> Bounds<T> {
    pub fn in_bounds(&self, p: Vec2<T>) -> bool {
        let zero = T::zero();
        p.x >= zero && p.x < self.w && p.y >= zero && p.y < self.h
    }
}
