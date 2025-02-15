use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(22);

fn next_secret(x: u32) -> u32 {
    let m = 16777216;

    let mut x = x as u64;
    x = (x ^ (x * 64)) % m;
    x = (x ^ (x / 32)) % m;
    x = (x ^ (x * 2048)) % m;
    x as u32
}

fn secret_sequence(secret: u32) -> u32 {
    (0..2000).fold(secret, |acc, _| next_secret(acc))
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .par_lines()
            .map(|s| s.parse::<u32>().unwrap())
            .map(|s| secret_sequence(s) as u64)
            .sum(),
    )
}

const WINDOW_ID_SPACE: usize = 2 << 19;

fn next_window_id(window_id: usize, delta: i32) -> usize {
    ((window_id << 5) | ((delta + 9) as usize)) & ((2 << 19) - 1)
}

fn process_batch(secrets: impl Iterator<Item = u32>) -> Vec<i32> {
    let mut prices_by_window = vec![0; WINDOW_ID_SPACE];
    let mut seen = vec![0; WINDOW_ID_SPACE];

    for (seller, secret) in secrets.enumerate() {
        let seller_id = seller + 1;
        let mut window_id: usize = 0;
        let mut value = secret;

        let mut previous_price: i32 = 0;
        for i in 0..2000 {
            let price = (value % 10) as i32;
            let delta = previous_price - price;

            window_id = next_window_id(window_id, delta);
            if i > 4 && seen[window_id] != seller_id {
                seen[window_id] = seller_id;
                prices_by_window[window_id] += price;
            }

            value = next_secret(value);
            previous_price = price;
        }
    }

    prices_by_window
}

pub fn part_two(input: &str) -> Option<u32> {
    fn merge_prices(mut prices1: Vec<i32>, prices2: Vec<i32>) -> Vec<i32> {
        prices1
            .iter_mut()
            .zip(prices2)
            .for_each(|(p1, p2)| *p1 += p2);
        prices1
    }

    let prices = input
        .lines()
        .collect_vec()
        .par_chunks(1200)
        .map(|chunk| process_batch(chunk.iter().map(|s| s.parse::<u32>().unwrap())))
        .reduce_with(merge_prices)
        .unwrap();

    Some((*prices.iter().max().unwrap()) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::iter::successors;

    #[test]
    fn test_example_secrets() {
        let secrets = successors(Some(123), |x| Some(next_secret(*x)))
            .dropping(1)
            .take(10)
            .collect_vec();
        assert_eq!(
            secrets,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
