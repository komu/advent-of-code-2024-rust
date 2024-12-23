use advent_of_code::collections::{extend, intersection};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

struct Graph<'a> {
    neighbors: HashMap<&'a str, HashSet<&'a str>>
}

impl <'a> Graph<'a> {
    fn parse(input: &str) -> Graph {
        let mut neighbors = HashMap::<&str, HashSet<&str>>::new();
        for line in input.lines() {
            let (a, b) = line.split_once('-').unwrap();
            neighbors.entry(a).or_default().insert(b);
            neighbors.entry(b).or_default().insert(a);
        }
        Graph { neighbors }
    }

    fn neighbors(&self, a: &str) -> &HashSet<&'a str> {
        self.neighbors.get(a).unwrap()
    }
}

fn find_maximum_cliques<'a>(g: &Graph<'a>) -> Vec<HashSet<&'a str>> {
    fn recurse<'a>(
        g: &Graph<'a>,
        r: HashSet<&'a str>,
        mut p: HashSet<&'a str>,
        mut x: HashSet<&'a str>,
        result: &mut Vec<HashSet<&'a str>>,
    ) {
        if p.is_empty() && x.is_empty() {
            result.push(r);
            return;
        }

        if let Some(&pivot) = p.iter().next().or(x.iter().next()) {
            let pivot_neighbors = g.neighbors(pivot);

            for v in p.clone() {
                if pivot_neighbors.contains(v) {
                    continue;
                }

                let ns = g.neighbors(v);
                recurse(
                    g,
                    extend(&r, v),
                    intersection(&p, ns),
                    intersection(&x, ns),
                    result,
                );
                p.remove(v);
                x.insert(v);
            }
        }
    }

    let mut result = Vec::<HashSet<&'a str>>::new();
    recurse(
        g,
        HashSet::new(),
        g.neighbors.keys().copied().collect(),
        HashSet::new(),
        &mut result,
    );
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let g = Graph::parse(input);

    let mut result = HashSet::<[&str; 3]>::new();
    for a in g.neighbors.keys().filter(|&x| x.starts_with('t')) {
        let neighbors = g.neighbors(a);
        for &b in neighbors {
            for &c in neighbors {
                if b < c && g.neighbors(c).contains(b) {
                    let mut r = [a, b, c];
                    r.sort();
                    result.insert(r);
                }
            }
        }
    }
    Some(result.len() as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let g = Graph::parse(input);
    Some(
        find_maximum_cliques(&g)
            .iter()
            .max_by_key(|x| x.len())
            .unwrap()
            .iter()
            .sorted()
            .join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
