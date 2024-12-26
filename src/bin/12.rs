use advent_of_code::char_grid::ByteGrid;
use advent_of_code::directions::CARDINAL_DIRECTIONS;
use advent_of_code::grid::Grid;
use advent_of_code::vec2::Vec2;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(12);

type Point = Vec2<i32>;

#[derive(Default)]
struct Region {
    points: HashSet<Point>,
}

impl Region {
    fn perimeter(&self) -> u32 {
        let mut adjacent: u32 = 0;
        for a in &self.points {
            for b in &self.points {
                if a < b && a.is_cardinal_neighbor(b) {
                    adjacent += 1;
                }
            }
        }

        4 * self.size() - 2 * adjacent
    }

    fn size(&self) -> u32 {
        self.points.len() as u32
    }

    fn sides(&self) -> u32 {
        // Divide the area into 2x2 blocks
        let deltas = [
            Vec2::new(0, 0),
            Vec2::new(0, 1),
            Vec2::new(1, 0),
            Vec2::new(1, 1),
        ];
        let blocks = self
            .points
            .iter()
            .flat_map(|&p| deltas.map(|d| p - d))
            .unique()
            .map(|p| deltas.map(|d| p + d))
            .collect_vec();

        let mut corners = 0;
        for block in blocks {
            let ps = block
                .iter()
                .filter(|&p| self.points.contains(p))
                .collect_vec();

            if ps.len() == 1 || ps.len() == 3 {
                corners += 1;
            } else if ps.len() == 2 && ps[0].is_diagonal_neighbor(ps[1]) {
                corners += 2;
            }
        }

        corners // number of sides is equal to number of corners
    }
}

/// Converts character grid to int-grid so that each region gets its own numeric non-zero id.
fn normalize_regions(grid: &ByteGrid) -> Grid<u32> {
    let uninitialized = 0;
    let mut result = Grid::new(grid.get_width(), grid.get_height(), uninitialized);

    let cardinal_neighbors = CARDINAL_DIRECTIONS.map(|d| d.to_vec());
    let mut id = 1;
    for p in grid.points() {
        if result[&p] == uninitialized {
            let value = grid[&p];

            result.flood(&p, id, &cardinal_neighbors, uninitialized, |p| {
                grid[p] == value
            });
            id += 1;
        }
    }

    result
}

fn extract_regions(input: &str) -> impl Iterator<Item = Region> {
    let grid = normalize_regions(&ByteGrid::new(input));

    let mut groups = HashMap::<u32, Region>::new();
    for p in grid.points() {
        let value = grid[&p];
        groups.entry(value).or_default().points.insert(p);
    }

    groups.into_iter().map(|(_, r)| r)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        extract_regions(input)
            .par_bridge()
            .map(|r| r.size() * r.perimeter())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        extract_regions(input)
            .par_bridge()
            .map(|r| r.size() * r.sides())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file_part;

    #[test]
    fn test_part_one_1() {
        assert_eq!(part_one(&read_file_part("examples", DAY, 1)), Some(140));
    }

    #[test]
    fn test_part_one_2() {
        assert_eq!(part_one(&read_file_part("examples", DAY, 2)), Some(772));
    }

    #[test]
    fn test_part_one_3() {
        assert_eq!(part_one(&read_file_part("examples", DAY, 3)), Some(1930));
    }

    #[test]
    fn test_part_two_1() {
        assert_eq!(part_two(&read_file_part("examples", DAY, 1)), Some(80));
    }

    #[test]
    fn test_part_two_2() {
        assert_eq!(part_two(&read_file_part("examples", DAY, 2)), Some(436));
    }

    #[test]
    fn test_part_two_3() {
        assert_eq!(part_two(&read_file_part("examples", DAY, 3)), Some(1206));
    }

    #[test]
    fn test_part_two_4() {
        assert_eq!(part_two(&read_file_part("examples", DAY, 4)), Some(236));
    }

    #[test]
    fn test_part_two_5() {
        assert_eq!(part_two(&read_file_part("examples", DAY, 5)), Some(368));
    }
}
