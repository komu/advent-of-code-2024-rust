use advent_of_code::char_grid::ByteGrid;
use advent_of_code::vec2::Vec2;
use hashbrown::HashSet;
use num::abs;

advent_of_code::solution!(20);

type Point = Vec2<i32>;

#[derive(Eq, PartialEq, Clone, Hash)]
struct Cheat {
    start: Point,
    end: Point,
}

fn count_over_threshold(input: &str, distance: i32, threshold: u32) -> u32 {
    let track = ByteGrid::new(input);
    let start = track.find(b'S').unwrap();
    let end = track.find(b'E').unwrap();

    let from_start = track.distances_from(start, |c| c != b'#');
    let from_end = track.distances_from(end, |c| c != b'#');
    let normal_cost = from_start[&end];

    let mut cheats = HashSet::<Cheat>::new();

    let mut result = 0;

    for start in track.points() {
        if track[&start] == b'#' { continue }

        for dy in -distance..=distance {
            let dx_range = distance - abs(dy);
            for dx in -dx_range..=dx_range {
                let end = Point::new(start.x + dx, start.y + dy);
                if !track.contains(&end) || track[&end] == b'#' { continue }

                let cheat_cost = (abs(dx) + abs(dy)) as i64;
                if cheat_cost > 1 {
                    let cost = cheat_cost + from_start[&start] as i64 + from_end[&end] as i64;
                    let gain = (normal_cost as i64) - cost;
                    if gain >= (threshold as i64) && cheats.insert(Cheat { start, end }) {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(count_over_threshold(input, 2, 100))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_over_threshold(input, 20, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(count_over_threshold(&advent_of_code::template::read_file("examples", DAY), 2, 2), 44);
        assert_eq!(count_over_threshold(&advent_of_code::template::read_file("examples", DAY), 2, 10), 10);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(count_over_threshold(&advent_of_code::template::read_file("examples", DAY), 20, 2), 3081);
        assert_eq!(count_over_threshold(&advent_of_code::template::read_file("examples", DAY), 20, 10), 2268);
    }
}
