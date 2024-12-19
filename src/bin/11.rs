use advent_of_code::numeric::count_digits;
use hashbrown::HashMap;
use rayon::prelude::*;

advent_of_code::solution!(11);

type Stone = u64;

fn solve(input: &str, total_rounds: u8) -> u64 {
    fn recurse(stone: Stone, rounds: u8, cache: &mut HashMap<(Stone, u8), u64>) -> u64 {
        if let Some(&result) = cache.get(&(stone, rounds)) {
            result
        } else {
            let result = if rounds == 0 {
                1
            } else if stone == 0 {
                recurse(1, rounds - 1, cache)
            } else {
                let digits = count_digits(stone);
                if digits % 2 == 0 {
                    let div = Stone::pow(10, digits / 2);
                    recurse(stone / div, rounds - 1, cache)
                        + recurse(stone % div, rounds - 1, cache)
                } else {
                    recurse(stone * 2024, rounds - 1, cache)
                }
            };
            cache.insert((stone, rounds), result);
            result
        }
    }

    input
        .par_split(' ')
        .map(|s| recurse(s.parse().unwrap(), total_rounds, &mut HashMap::new()))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
