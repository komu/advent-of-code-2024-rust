use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::RangeInclusive;

type Vec2 = advent_of_code::vec2::Vec2<i16>;
type Bounds = advent_of_code::bounds::Bounds<i16>;

advent_of_code::solution!(8);

fn antinodes(antennas: &[Vec2], range: RangeInclusive<i16>, bounds: Bounds, result: &mut BTreeSet<Vec2>) {
    for (a, b) in antennas.iter().copied().tuple_combinations() {
        let v = a - b;

        for (n, sign) in [(a, 1), (b, -1)] {
            for i in range.clone() {
                let p = n + sign * i * v;
                if bounds.in_bounds(p) {
                    result.insert(p);
                } else {
                    break;
                }
            }
        }
    }
}

fn solve(input: &str, range: RangeInclusive<i16>) -> u32 {
    let mut antennas: BTreeMap<char, Vec<Vec2>> = BTreeMap::new();

    let mut max_row = 0;
    let mut max_col = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '.' {
                let p = Vec2::new(x as i16, y as i16);
                antennas
                    .entry(c)
                    .and_modify(|v| v.push(p))
                    .or_insert_with(|| vec![p]);
            }
            max_col = y;
        }
        max_row = y;
    }
    let bounds = Bounds::new((max_col + 1) as i16, (max_row + 1) as i16);

    let mut result = BTreeSet::<Vec2>::new();
    for (_, points) in antennas.iter() {
        antinodes(points, range.clone(), bounds, &mut result);
    }

    result.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 1..=1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 0..=i16::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
