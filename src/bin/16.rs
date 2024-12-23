use advent_of_code::char_grid::ByteGrid;
use advent_of_code::directions::CardinalDirection;
use advent_of_code::directions::CardinalDirection::East;
use advent_of_code::shortest_path::{nodes_on_all_shortest_paths, shortest_path_len, Graph};
use advent_of_code::vec2::Vec2;
use itertools::Itertools;

advent_of_code::solution!(16);

type Point = Vec2<i32>;

struct Maze<'a> {
    start: Point,
    end: Point,
    grid: ByteGrid<'a>,
}

impl Maze<'_> {
    fn parse(input: &str) -> Maze {
        let grid = ByteGrid::new(input);
        let start = grid.find(b'S').expect("no start");
        let end = grid.find(b'E').expect("no end");

        Maze { start, end, grid }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct MazeState {
    p: Point,
    d: CardinalDirection,
}

impl Graph for Maze<'_> {
    type Node = MazeState;

    fn is_solution(&self, node: &MazeState) -> bool {
        node.p == self.end
    }

    fn collect_neighbors(&self, node: &MazeState, neighbors: &mut Vec<(MazeState, u32)>) {
        assert!(self.grid.contains(&node.p));

        let forward = node.p + node.d.to_vec();

        if self.grid.contains(&forward) && self.grid[&forward] != b'#' {
            neighbors.push((MazeState { p: forward, d: node.d }, 1));
        }

        neighbors.push((MazeState { p: node.p, d: node.d.clockwise() }, 1000));
        neighbors.push((MazeState { p: node.p, d: node.d.counter_clockwise() }, 1000));
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::parse(input);
    let start_state = MazeState { p: maze.start, d: East };
    shortest_path_len(&maze, start_state).map(|(_, len)| len)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = Maze::parse(input);
    let start_state = MazeState { p: maze.start, d: East };
    let (_, states) = nodes_on_all_shortest_paths(&maze, start_state);
    let points = states.iter().map(|s| s.p).unique().count();
    Some(points as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }
}
