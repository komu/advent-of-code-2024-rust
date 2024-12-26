use rayon::prelude::*;

advent_of_code::solution!(7);

fn is_satisfiable(total: u64, xs: &[u64], part2: bool) -> bool {
    fn recurse<I: Iterator<Item = u64> + Clone>(
        total: u64,
        acc: u64,
        mut iter: I,
        part2: bool,
    ) -> bool {
        if let Some(x) = iter.next() {
            acc < total
                && ((part2 && recurse(total, concat(acc, x), iter.clone(), part2))
                    || recurse(total, acc * x, iter.clone(), part2)
                    || recurse(total, acc + x, iter, part2))
        } else {
            acc == total
        }
    }

    recurse(total, xs[0], xs[1..].iter().copied(), part2)
}

fn concat(x: u64, y: u64) -> u64 {
    // a sane implementation would be: let m = 10_u64.pow(y.ilog10() + 1);
    let m = ((y >= 10) as u64 * 90 + (y >= 100) as u64 * 900 + (y >= 1000) as u64 * 9000) + 10;
    m * x + y
}

fn solve(input: &str, part2: bool) -> Option<u64> {
    fn handle(s: &str, part2: bool) -> u64 {
        let (total, xs) = s.split_once(": ").unwrap();
        let total: u64 = total.parse().unwrap();
        let xs: Vec<u64> = xs.split(' ').map(|s| s.parse().unwrap()).collect();

        if is_satisfiable(total, &xs, part2) {
            total
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
