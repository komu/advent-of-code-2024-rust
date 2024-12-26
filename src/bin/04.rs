use advent_of_code::char_grid::ByteGrid;
use advent_of_code::vec2::{Vec2, DIRECTIONS};
use itertools::Itertools;

advent_of_code::solution!(4);

fn contains_xmas(grid: &ByteGrid, p: &Vec2<i32>, d: &Vec2<i32>) -> bool {
    "XMAS"
        .bytes()
        .enumerate()
        .all(|(i, c)| grid[&(*p + (i as i32) * *d)] == c)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = ByteGrid::new(input);
    Some(
        grid.points()
            .map(|p| {
                DIRECTIONS
                    .iter()
                    .filter(|&d| contains_xmas(&grid, &p, d))
                    .count() as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = ByteGrid::new(input);

    let deltas = [
        Vec2::new(0, 0),
        Vec2::new(-1, -1),
        Vec2::new(1, -1),
        Vec2::new(1, 1),
        Vec2::new(-1, 1),
    ];
    let valid_signatures =
        ["AMMSS", "AMSSM", "ASMMS", "ASSMM"].map(|s| s.bytes().collect::<Vec<_>>());

    Some(
        grid.points()
            .filter(|p| {
                let signature = deltas.iter().map(|i| grid[&(*p + *i)]).collect::<Vec<_>>();
                valid_signatures.iter().contains(&&signature)
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
