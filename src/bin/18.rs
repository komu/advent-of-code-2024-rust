use advent_of_code::binary_search::binary_search;
use advent_of_code::directions::CARDINAL_DIRECTIONS;
use advent_of_code::vec2::Vec2;
use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::VecDeque;

advent_of_code::solution!(18);

type Point = Vec2<i32>;

fn path_length(corrupted: &HashSet<Point>, size: i32) -> Option<u32> {
    let start = Point::new(0, 0);
    let end = Point::new(size, size);

    let mut seen = HashSet::<Point>::new();
    seen.insert(start);

    let mut queue = VecDeque::<(Point, u32)>::new();
    queue.push_back((Point::new(0, 0), 0));

    while let Some((u, cost)) = queue.pop_front() {
        if u == end {
            return Some(cost);
        } else {
            for d in CARDINAL_DIRECTIONS {
                let p = u + d.to_vec();
                if p.x >= 0 && p.x <= size && p.y >= 0 && p.y <= size && !corrupted.contains(&p) {
                    if seen.insert(p) {
                        queue.push_back((p, cost + 1));
                    }
                }
            }
        }
    }

    None
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

    let i = binary_search(0..points.len(), |&i| {
        let corrupted_points = points.iter().take(i).cloned().collect::<HashSet<_>>();
        path_length(&corrupted_points, size).is_none()
    })
    .unwrap();

    let p = points[i - 1];
    format!("{},{}", p.x, p.y)
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
