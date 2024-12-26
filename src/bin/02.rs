use rayon::prelude::*;
use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn is_safe(line: &str, tolerate: bool) -> bool {
    let values = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    is_safe_in(&values, 1..=3, tolerate) || is_safe_in(&values, -3..=-1, tolerate)
}

fn is_safe_in(xs: &[i32], range: RangeInclusive<i32>, tolerate: bool) -> bool {
    let mut may_ignore = tolerate;

    let mut prev = xs[0];
    for &value in &xs[1..] {
        if range.contains(&(value - prev)) {
            prev = value
        } else if may_ignore {
            may_ignore = false
        } else {
            return false;
        }
    }
    true
}

fn solve(input: &str, tolerate: bool) -> u32 {
    input.par_lines().filter(|s| is_safe(s, tolerate)).count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
