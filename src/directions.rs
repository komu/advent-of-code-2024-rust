use crate::vec2::Vec2;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

pub const CARDINAL_DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::North,
    CardinalDirection::East,
    CardinalDirection::South,
    CardinalDirection::West,
];

impl CardinalDirection {
    pub fn to_vec(self) -> Vec2<i32> {
        use CardinalDirection::*;
        match self {
            North => Vec2::new(0, -1),
            East => Vec2::new(1, 0),
            South => Vec2::new(0, 1),
            West => Vec2::new(-1, 0),
        }
    }

    pub fn clockwise(self) -> Self {
        use CardinalDirection::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn counter_clockwise(self) -> Self {
        use CardinalDirection::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn from_code(code: char) -> Self {
        use CardinalDirection::*;
        match code {
            '^' => North,
            '>' => East,
            'v' => South,
            '<' => West,
            _ => panic!("invalid direction code: {}", code),
        }
    }
}
