use advent_of_code::char_grid::ByteGrid;
use advent_of_code::directions::CARDINAL_DIRECTIONS;
use advent_of_code::vec2::Vec2;
use hashbrown::HashSet;

advent_of_code::solution!(10);

type Point = Vec2<i32>;

fn collect_path_ends<T: Extend<Point>>(map: &ByteGrid, start: Point, result: &mut T) {
    let mut queue = Vec::with_capacity(32);

    queue.push((start, b'0'));

    while let Some((point, expected_level)) = queue.pop() {
        if map[&point] == expected_level {
            if expected_level == b'9' {
                result.extend(Some(point));
            } else {
                for d in CARDINAL_DIRECTIONS {
                    queue.push((point + d.to_vec(), expected_level + 1));
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = ByteGrid::new(input);
    let mut points = HashSet::<Point>::new();
    Some(
        map.find_all(b'0')
            .map(|p| {
                points.clear();
                collect_path_ends(&map, p, &mut points);
                points.len()
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = ByteGrid::new(input);
    let mut points = Vec::<Point>::new();
    for p in map.find_all(b'0') {
        collect_path_ends(&map, p, &mut points)
    }
    Some(points.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
