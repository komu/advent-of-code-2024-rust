use advent_of_code::directions::CARDINAL_DIRECTIONS;
use advent_of_code::shortest_path::{shortest_path_len, Graph};
use advent_of_code::vec2::Vec2;
use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(18);

type Point = Vec2<i32>;

struct Corrupted<'a> {
    corrupted: &'a HashSet<Point>,
    target: Point,
    size: i32,
}

impl<'a> Graph for Corrupted<'a> {
    type Node = Point;

    fn is_solution(&self, &node: &Self::Node) -> bool {
        node == self.target
    }

    fn collect_neighbors(&self, &node: &Self::Node, neighbors: &mut Vec<(Self::Node, u32)>) {
        // TODO: optimize for uniform cost
        for d in CARDINAL_DIRECTIONS {
            let p = node + d.to_vec();
            if self.is_in_bounds(&p) && !self.corrupted.contains(&p) {
                neighbors.push((p, 1));
            }
        }
    }
}

impl Corrupted<'_> {
    fn is_in_bounds(&self, &p: &Point) -> bool {
        p.x >= 0 && p.x <= self.size && p.y >= 0 && p.y <= self.size
    }
}

fn path_length(corrupted: &HashSet<Point>, size: i32) -> Option<u32> {
    let corrupted = Corrupted {
        corrupted,
        target: Point::new(size, size),
        size,
    };

    shortest_path_len(&corrupted, Point::new(0, 0)).map(|(_, cost)| cost)
}

fn parse(input: &str) -> impl Iterator<Item = Point> + '_ {
    input.lines().map(|s| s.parse().unwrap())
}

fn solve1(input: &str, size: i32, take: usize) -> u32 {
    let corrupted = parse(input).take(take).collect::<HashSet<_>>();
    path_length(&corrupted, size).expect("no path")
}

fn solve2(input: &str, size: i32) -> String {
    let points = parse(input).collect_vec();

    // TODO: binary search for position
    let mut corrupted = HashSet::new();
    for (i, corrupted_point) in points.iter().enumerate() {
        println!("{}/{}", i, points.len());
        corrupted.insert(*corrupted_point);
        if path_length(&corrupted, size).is_none() {
            let p = points[i];
            return format!("{},{}", p.x, p.y)
        }
    }

    panic!("did not found");
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve1(input, 70, 1024))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(solve2(input, 70))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(solve1(&input, 6, 12), 22);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(solve2(&input, 6), "6,1");
    }
}
