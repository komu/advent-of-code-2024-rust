use hashbrown::HashSet;
use rayon::prelude::*;

advent_of_code::solution!(19);

struct PatternData<'a> {
    patterns: HashSet<&'a str>,
    max_pattern_len: usize,
}

impl PatternData<'_> {
    fn is_possible(&self, design: &str) -> bool {
        if design.is_empty() {
            true
        } else {
            let mut possible = false;

            for i in 1..=design.len() {
                let (prefix, suffix) = design.split_at(i);

                if self.patterns.contains(&prefix) && self.is_possible(suffix) {
                    possible = true;
                    break;
                }
            }

            possible
        }
    }

    fn ways_to_make(&self, design: &str) -> u64 {
        let mut dp = vec![0; design.len() + 1];
        dp[0] = 1;

        for i in 1..=design.len() {
            let prefix = &design[..i];

            for pat_len in 1..=self.max_pattern_len.min(i) {
                if self.patterns.contains(&prefix[i - pat_len..]) {
                    dp[i] += dp[i - pat_len];
                }
            }
        }

        dp[design.len()]
    }
}

fn parse(input: &str) -> (PatternData, impl ParallelIterator<Item = &str>) {
    let (patterns_str, designs) = input.split_once("\n\n").unwrap();

    let patterns: HashSet<_> = patterns_str.split(", ").collect();
    let max_pattern_len = patterns.iter().map(|it| it.len()).max().unwrap();

    let pattern_data = PatternData {
        patterns,
        max_pattern_len,
    };
    (pattern_data, designs.par_lines())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (data, designs) = parse(input);
    let count = designs.filter(|d| data.is_possible(d)).count();
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (data, designs) = parse(input);
    Some(designs.map(|d| data.ways_to_make(d)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
