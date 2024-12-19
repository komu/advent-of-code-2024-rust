use std::collections::HashMap;

advent_of_code::solution!(19);

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (s1, s2) = input.split_once("\n\n").unwrap();

    let patterns = s1.split(", ").collect::<Vec<_>>();
    let designs = s2.lines().collect::<Vec<_>>();

    (patterns, designs)
}

fn ways_to_make(design: &str, patterns: &[&str]) -> u64 {
    ways_to_make_with(design, patterns, &mut HashMap::new())
}

fn ways_to_make_with<'a>(
    design: &'a str,
    patterns: &'a [&'a str],
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        1
    } else if let Some(&value) = cache.get(design) {
        value
    } else {
        let ways = patterns
            .iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|&pattern| ways_to_make_with(&design[pattern.len()..], patterns, cache))
            .sum();

        cache.insert(design, ways);
        ways
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns, designs) = parse(input);

    Some(
        designs
            .iter()
            .filter(|&it| ways_to_make(it, &patterns) != 0)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, designs) = parse(input);

    Some(designs.iter().map(|&it| ways_to_make(it, &patterns)).sum())
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
