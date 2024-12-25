use num::abs;
use std::fmt::Debug;
use std::str::FromStr;
use anyhow::{anyhow, bail};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T: std::ops::Add<T, Output = T>> std::ops::Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: std::ops::AddAssign<T>> std::ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Vec2<i32> {
    pub fn squared_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }

    pub fn is_cardinal_neighbor(&self, other: &Self) -> bool {
        self.squared_distance(other) == 1
    }

    pub fn is_diagonal_neighbor(&self, other: &Self) -> bool {
        abs(self.x - other.x) == 1 && abs(self.y - other.y) == 1
    }
}

impl<T: std::ops::Sub<T, Output = T>> std::ops::Sub for Vec2<T> {
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

impl<T: FromStr> FromStr for Vec2<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(',') {
            Ok(Vec2 {
                x: x.parse().map_err(|_| anyhow!("failed to parse x coordinate from '{s}'"))?,
                y: y.parse().map_err(|_| anyhow!("failed to parse y coordinate from '{s}'"))?,
            })
        } else {
            bail!("missing comma in point '{s}'");
        }
    }
}
