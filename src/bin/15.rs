use advent_of_code::char_grid::ByteGrid;
use advent_of_code::directions::CardinalDirection;
use advent_of_code::directions::CardinalDirection::{East, North, South, West};
use advent_of_code::grid::Grid;
use advent_of_code::vec2::Vec2;

advent_of_code::solution!(15);

type Point = Vec2<i32>;

const EMPTY: usize = usize::MAX;

#[derive(Clone)]
struct Box {
    west: Point,
    east: Point,
}

impl Box {

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
    index_cache: Grid<usize>,
    robot: Point,
}

impl Warehouse<'_> {
    fn has_wall(&self, p: &Point) -> bool {
        self.grid[p] == b'#'
    }

    fn can_move_towards(
        &self,
        p: Point,
        d: CardinalDirection,
        moved_indices: &mut Vec<usize>,
    ) -> bool {
        self.can_move_towards_rec(p, d, moved_indices)
    }

    fn find_box_index_for(&self, p: Point) -> Option<usize> {
        let index = self.index_cache[&p];
        if index != EMPTY {
            Some(index)
        } else {
            None
        }
    }

    fn push_box(&mut self, index: usize, d: CardinalDirection) {
        let b = &mut self.boxes[index];

        self.index_cache[&b.west] = EMPTY;
        self.index_cache[&b.east] = EMPTY;
        b.push(d);
        self.index_cache[&b.west] = index;
        self.index_cache[&b.east] = index;
    }

    fn can_move_towards_rec(
        &self,
        p: Point,
        d: CardinalDirection,
        moved_indices: &mut Vec<usize>,
    ) -> bool {
        let target = p + d.to_vec();

        if self.has_wall(&target) {
            return false;
        }

        if let Some(index) = self.find_box_index_for(target) {
            if !moved_indices.contains(&index) {
                moved_indices.push(index);
            }

            let b = &self.boxes[index];
            match d {
                North | South => {
                    if b.is_wide() {
                        self.can_move_towards_rec(b.west, d, moved_indices)
                            && self.can_move_towards_rec(b.east, d, moved_indices)
                    } else {
                        self.can_move_towards_rec(b.west, d, moved_indices)
                    }
                }
                West => self.can_move_towards_rec(b.west, d, moved_indices),
                East => self.can_move_towards_rec(b.east, d, moved_indices),
            }
        } else {
            true
        }
    }

    fn parse(input: &str) -> Warehouse {
        let grid = ByteGrid::new(input);
        let mut index_cache = Grid::new(grid.get_width(), grid.get_height(), EMPTY);

        let mut boxes = Vec::with_capacity(grid.get_width() * grid.get_height() / 2);
        let east = East.to_vec();
        let mut i = 0;
        for p in grid.points() {
            let v = grid[&p];
            match v {
                b'O' => {
                    boxes.push(Box { west: p, east: p });
                    index_cache[&p] = i;
                    i += 1;
                }
                b'[' => {
                    boxes.push(Box {
                        west: p,
                        east: p + east,
                    });
                    index_cache[&p] = i;
                    index_cache[&(p + east)] = i;
                    i += 1;
                }
                _ => {}
            }
        }

        let robot = grid.find(b'@').expect("no robot");
        Warehouse {
            boxes,
            grid,
            robot,
            index_cache,
        }
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

    let mut moved_indices = Vec::with_capacity(16);
    for mv in moves {
        moved_indices.clear();

        if warehouse.can_move_towards(robot, mv, &mut moved_indices) {
            for &index in moved_indices.iter().rev() {
                warehouse.push_box(index, mv);
            }
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
