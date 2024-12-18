use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

fn parse(input: &str) -> (OrderingRules, Vec<Vec<i32>>) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let mut before_than = HashMap::<i32, HashSet<i32>>::new();
    for rule in rules_str.lines() {
        let (x, y) = rule.split_once('|').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        before_than.entry(x).or_default().insert(y);
    }

    let updates: Vec<Vec<i32>> = updates_str
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    (OrderingRules { before_than }, updates)
}

struct OrderingRules {
    before_than: HashMap<i32, HashSet<i32>>,
}

impl OrderingRules {
    fn comes_before(&self, x: i32, y: i32) -> bool {
        self.before_than
            .get(&x)
            .map(|s| s.contains(&y))
            .unwrap_or(false)
    }

    fn sort_by_rules(&self, update: &[i32]) -> Vec<i32> {
        update
            .iter()
            .copied()
            .sorted_by(|&x, &y| -> Ordering {
                if self.comes_before(x, y) {
                    Ordering::Less
                } else if self.comes_before(y, x) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering_rules, updates) = parse(input);

    let mut result = 0;
    for update in &updates {
        if ordering_rules.sort_by_rules(update).eq(update) {
            result += update[update.len() / 2];
        }
    }
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering_rules, updates) = parse(input);

    Some(
        updates
            .iter()
            .map(|update| (update, ordering_rules.sort_by_rules(update)))
            .filter(|(original, sorted)| !original.eq(&sorted))
            .map(|(_, sorted)| sorted[sorted.len() / 2])
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
