use rayon::prelude::*;
use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn is_safe(line: &str, tolerate: bool) -> bool {
    let xs = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    if tolerate {
        (0..xs.len()).any(|i| is_safe_in_either_direction(&xs, i))
    } else {
        is_safe_in_either_direction(&xs, usize::MAX)
    }
}

fn is_safe_in_either_direction(xs: &[i32], ignored: usize) -> bool {
    is_safe_towards(xs, 1..=3, ignored) || is_safe_towards(xs, -3..=-1, ignored)
}

fn is_safe_towards(xs: &[i32], range: RangeInclusive<i32>, ignored: usize) -> bool {
    if ignored == 0 {
        return is_safe_towards(&xs[1..], range, usize::MAX);
    }

    let mut prev = xs[0];
    for (i, &value) in xs[1..].iter().enumerate() {
        if i + 1 == ignored {
        } else if range.contains(&(value - prev)) {
            prev = value
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
