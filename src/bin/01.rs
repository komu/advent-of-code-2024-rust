use itertools::Itertools;
use num::abs;

advent_of_code::solution!(1);

fn parse(s: &str) -> (Vec<i32>, Vec<i32>) {
    s.lines()
        .map(|s| {
            let (x, y) = s.split_once("   ").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut xs, mut ys) = parse(input);

    xs.sort_unstable();
    ys.sort_unstable();

    Some(
        xs.iter()
            .zip(ys.iter())
            .map(|(&x, &y)| abs(x - y))
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (xs, ys) = parse(input);

    let ys_counts = ys.iter().copied().counts();

    Some(
        xs.iter()
            .map(|x| x * (ys_counts.get(x).copied().unwrap_or(0) as i32))
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
