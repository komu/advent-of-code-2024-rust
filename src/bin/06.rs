use advent_of_code::char_grid::ByteGrid;
use advent_of_code::directions::CardinalDirection;
use hashbrown::HashSet;
use itertools::Itertools;
use rayon::prelude::*;

type Vec2 = advent_of_code::vec2::Vec2<i32>;

advent_of_code::solution!(6);

fn has_loop(grid: &ByteGrid, obstruction: &Vec2, start: Vec2) -> bool {
    let mut d = CardinalDirection::North;
    let mut p = start;
    let mut seen = HashSet::<(Vec2, CardinalDirection)>::new();

    while grid.contains(&p) {
        let mut next = p + d.to_vec();

        while grid[&next] == b'#' || next == *obstruction {
            if !seen.insert((p, d)) {
                return true;
            }

            d = d.clockwise();
            next = p + d.to_vec();
        }

        p = next;
    }

    false
}

fn points_on_path(grid: &ByteGrid, start: Vec2) -> HashSet<Vec2> {
    let mut seen = HashSet::<Vec2>::new();

    let mut d = CardinalDirection::North;
    let mut p = start;

    while grid.contains(&p) {
        seen.insert(p);

        while grid[&(p + d.to_vec())] == b'#' {
            d = d.clockwise();
        }

        p += d.to_vec();
    }

    seen
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = ByteGrid::new(input);
    let start = map.find(b'^').unwrap();

    Some(points_on_path(&map, start).len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = ByteGrid::new(input);
    let start = map.find(b'^').unwrap();

    let mut candidate_obstacles = points_on_path(&map, start);
    candidate_obstacles.remove(&start);

    Some(
        candidate_obstacles
            .iter()
            .collect_vec()
            .par_iter()
            .filter(|&it| has_loop(&map, it, start))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
