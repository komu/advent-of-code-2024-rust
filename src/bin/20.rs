use advent_of_code::char_grid::ByteGrid;
use advent_of_code::vec2::Vec2;
use num::abs;
use rayon::prelude::*;

advent_of_code::solution!(20);

fn count_over_threshold(input: &str, distance: i32, threshold: u32) -> u32 {
    let track = ByteGrid::new(input);
    let track_start = track.find(b'S').unwrap();
    let track_end = track.find(b'E').unwrap();

    let from_start = track.distances_from(track_start, |c| c != b'#');
    let from_end = track.distances_from(track_end, |c| c != b'#');
    let normal_cost = from_start[&track_end];

    track
        .par_points()
        .map(|start| (start, from_start[&start]))
        .filter(|(_, cost)| *cost < u32::MAX)
        .map(|(start, cost_from_start)| {
            let mut result = 0;
            for dy in -distance..=distance {
                let dx_range = distance - abs(dy);
                for dx in -dx_range..=dx_range {
                    let end = Vec2::new(start.x + dx, start.y + dy);
                    let cost_from_end = from_end.get(&end).copied().unwrap_or(u32::MAX);
                    if cost_from_end < u32::MAX {
                        let cheat_cost = (abs(dx) + abs(dy)) as u32;
                        let cost = cheat_cost + cost_from_start + cost_from_end;
                        if cheat_cost > 1 && cost < normal_cost && (normal_cost - cost) >= threshold {
                            result += 1;
                        }
                    }
                }
            }
            result
        })
        .sum()
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
        assert_eq!(
            count_over_threshold(&advent_of_code::template::read_file("examples", DAY), 2, 2),
            44
        );
        assert_eq!(
            count_over_threshold(&advent_of_code::template::read_file("examples", DAY), 2, 10),
            10
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            count_over_threshold(&advent_of_code::template::read_file("examples", DAY), 20, 2),
            3081
        );
        assert_eq!(
            count_over_threshold(
                &advent_of_code::template::read_file("examples", DAY),
                20,
                10
            ),
            2268
        );
    }
}
