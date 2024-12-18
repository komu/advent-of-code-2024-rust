use std::fmt::Debug;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl <T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl <T : std::ops::Add<T, Output = T>> std::ops::Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl <T : std::ops::Sub<T, Output = T>> std::ops::Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<Vec2<i16>> for i16 {
    type Output = Vec2<i16>;

    fn mul(self, rhs: Vec2<i16>) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Mul<Vec2<i32>> for i32 {
    type Output = Vec2<i32>;

    fn mul(self, rhs: Vec2<i32>) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

pub const DIRECTIONS: [Vec2<i32>; 8] = [
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: -1 },
    Vec2 { x: -1, y: 1 },
    Vec2 { x: 1, y: -1 },
    Vec2 { x: 1, y: 1 },
];
