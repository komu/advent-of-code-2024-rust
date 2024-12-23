use advent_of_code::char_grid::ByteGrid;
use advent_of_code::directions::CardinalDirection;
use advent_of_code::directions::CardinalDirection::{East, North, South, West};
use advent_of_code::vec2::Vec2;

advent_of_code::solution!(15);

type Point = Vec2<i32>;

#[derive(Clone)]
struct Box {
    west: Point,
    east: Point,
}

impl Box {
    fn contains(&self, p: Point) -> bool {
        p == self.west || p == self.east
    }

    fn push(&mut self, d: CardinalDirection) {
        self.west += d.to_vec();
        self.east += d.to_vec();
    }

    fn is_wide(&self) -> bool {
        self.west != self.east
    }

    fn gps(&self) -> i32 {
        100 * self.west.y + self.west.x
    }
}

struct Warehouse<'a> {
    boxes: Vec<Box>,
    grid: ByteGrid<'a>,
    robot: Point,
}

impl Warehouse<'_> {
    fn has_wall(&self, p: &Point) -> bool {
        self.grid[p] == b'#'
    }

    fn can_move_towards(&self, p: Point, d: CardinalDirection) -> bool {
        let target = p + d.to_vec();

        if self.has_wall(&target) {
            return false;
        }

        if let Some(b) = self.boxes.iter().find(|b| b.contains(target)) {
            match d {
                North | South => {
                    if b.is_wide() {
                        self.can_move_towards(b.west, d) && self.can_move_towards(b.east, d)
                    } else {
                        self.can_move_towards(b.west, d)
                    }
                }
                West => self.can_move_towards(b.west, d),
                East => self.can_move_towards(b.east, d),
            }
        } else {
            true
        }
    }

    fn push_towards(&mut self, p: Point, d: CardinalDirection) {
        let target = p + d.to_vec();

        if let Some(index) = self.boxes.iter().position(|b| b.contains(target)) {
            let b = &self.boxes[index];
            let east = b.east;
            let west = b.west;

            match d {
                North | South => {
                    if b.is_wide() {
                        self.push_towards(west, d);
                        self.push_towards(east, d)
                    } else {
                        self.push_towards(east, d);
                    }
                }
                West => self.push_towards(west, d),
                East => self.push_towards(east, d),
            }

            self.boxes[index].push(d);
        }
    }

    fn parse(input: &str) -> Warehouse {
        let grid = ByteGrid::new(input);

        let mut boxes = Vec::with_capacity(grid.get_width() * grid.get_height() / 2);
        let east = East.to_vec();
        for p in grid.points() {
            let v = grid[&p];
            match v {
                b'O' => boxes.push(Box { west: p, east: p }),
                b'[' => boxes.push(Box {
                    west: p,
                    east: p + east,
                }),
                _ => {}
            }
        }

        let robot = grid.find(b'@').expect("no robot");
        Warehouse { boxes, grid, robot }
    }
}

fn solve(input: &str) -> u32 {
    let (p1, p2) = input.split_once("\n\n").unwrap();

    let mut warehouse = Warehouse::parse(p1);
    let moves = p2
        .chars()
        .filter(|&c| c != '\n')
        .map(CardinalDirection::from_code);
    let mut robot = warehouse.robot;

    for mv in moves {
        if warehouse.can_move_towards(robot, mv) {
            warehouse.push_towards(robot, mv);
            robot += mv.to_vec();
        }
    }

    warehouse.boxes.iter().map(|b| b.gps()).sum::<i32>() as u32
}

fn widen(input: &str) -> String {
    input
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(&widen(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_complex() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(618));
    }

    #[test]
    fn test_part_two_complex() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
