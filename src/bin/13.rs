use advent_of_code::numeric::det;
use itertools::Itertools;

type Vec2 = advent_of_code::vec2::Vec2<i64>;

advent_of_code::solution!(13);

fn tokens(a: Vec2, b: Vec2, c: Vec2) -> i64 {
    // Cramer's rule
    let det_cb = det(c, b);
    let det_ac = det(a, c);
    let det_ab = det(a, b);

    if det_ab != 0 && det_cb % det_ab == 0 && det_ac % det_ab == 0 {
        let x = det_cb / det_ab;
        let y = det_ac / det_ab;
        3 * x + y
    } else {
        0
    }
}

fn solve(input: &str, offset: Vec2) -> i64 {
    fn parse_vec(s: &str) -> Vec2 {
        let x = s.find("X").unwrap() + 2;
        let y = s.find("Y").unwrap() + 2;
        let c = s.find(",").unwrap();

        Vec2::new(s[x..c].parse().unwrap(), s[y..].parse().unwrap())
    }

    input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut chunk| chunk.next_tuple().unwrap())
        .map(|(a, b, c)| tokens(parse_vec(a), parse_vec(b), parse_vec(c) + offset))
        .sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, Vec2::new(0, 0)))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, Vec2::new(10000000000000, 10000000000000)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
