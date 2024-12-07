use anyhow::anyhow;
use itertools::Itertools;
use rayon::prelude::*;
use std::str::FromStr;

advent_of_code::solution!(7);

struct Equation {
    total: u64,
    xs: Vec<u64>,
}

impl Equation {
    fn recurse<I: Iterator<Item = u64> + Clone>(&self, acc: u64, mut iter: I, part2: bool) -> bool {
        if let Some(x) = iter.next() {
            acc < self.total
                && ((part2 && self.recurse(concat(acc, x), iter.clone(), part2))
                    || self.recurse(acc * x, iter.clone(), part2)
                    || self.recurse(acc + x, iter, part2))
        } else {
            acc == self.total
        }
    }

    fn is_satisfiable(&self, part2: bool) -> bool {
        self.recurse(self.xs[0], self.xs[1..].iter().copied(), part2)
    }
}

fn concat(x: u64, y: u64) -> u64 {
    // a sane implementation would be: let m = 10_u64.pow(y.ilog10() + 1);
    let m = ((y >= 10) as u64 * 90 + (y >= 100) as u64 * 900 + (y >= 1000) as u64 * 9000) + 10;
    m * x + y
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (total, xs) = s.split_once(": ").ok_or(anyhow!("can't split"))?;

        Ok(Equation {
            total: total.parse()?,
            xs: xs.split(' ').map(str::parse).try_collect()?,
        })
    }
}

fn solve(input: &str, part2: bool) -> Option<u64> {
    fn handle(s: &str, part2: bool) -> u64 {
        let e = s.parse::<Equation>().unwrap();
        if e.is_satisfiable(part2) {
            e.total
        } else {
            0
        }
    }

    Some(input.par_lines().map(|s| handle(s, part2)).sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
