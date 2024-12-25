use itertools::Itertools;
use advent_of_code::collections::transpose;

advent_of_code::solution!(25);

fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for entry in input.split("\n\n") {
        let chars = entry.lines().map(|s|s.chars().collect_vec()).collect_vec();
        let transposed = transpose(chars);
        let heights = transposed.iter().map(|row| row.iter().filter(|c| **c == '#').count() as u32).collect_vec();
        if entry.starts_with(".....") {
            keys.push(heights);
        } else if entry.starts_with("#####") {
            locks.push(heights);
        } else {
            panic!("invalid entry {entry}")
        }
    }

    (locks, keys)
}

fn overlaps(key: &[u32], lock: &[u32]) -> bool {
    key.iter().zip(lock.iter()).all(|(x,y)|  x + y < 8)
}

pub fn part_one(input: &str) -> Option<u32> {

    let (locks, keys) = parse(input);

    let sum = locks.iter().map(|l| keys.iter().filter(|k| overlaps(l, k)).count()).sum::<usize>();

    Some(sum as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
