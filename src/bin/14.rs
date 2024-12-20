use advent_of_code::vec2::DIRECTIONS;
use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(14);

type Vec2 = advent_of_code::vec2::Vec2<i32>;

struct Robot {
    p: Vec2,
    v: Vec2,
}

struct RobotSpace {
    w: i32,
    h: i32,
    robots: Vec<Robot>,
}

impl RobotSpace {
    fn tick(&mut self) {
        let w = self.w;
        let h = self.h;
        for robot in &mut self.robots {
            robot.p = Vec2::new(
                (robot.p.x + robot.v.x + w) % w,
                (robot.p.y + robot.v.y + h) % h,
            )
        }
    }

    fn quadrant_of(&self, p: Vec2) -> i32 {
        let mid_x = self.w / 2;
        let mid_y = self.h / 2;

        if p.x < mid_x && p.y < mid_y {
            1
        } else if p.x > mid_x && p.y < mid_y {
            2
        } else if p.x < mid_x && p.y > mid_y {
            3
        } else if p.x > mid_x && p.y > mid_y {
            4
        } else {
            0
        }
    }

    fn has_tree(&self) -> bool {
        let points = self.robots.iter().map(|r| r.p).collect::<HashSet<_>>();

        self.robots
            .iter()
            .any(|r| DIRECTIONS.iter().all(|&d| points.contains(&(r.p + d))))
    }

    fn checksum(&self) -> u32 {
        self.robots
            .iter()
            .counts_by(|r| self.quadrant_of(r.p))
            .into_iter()
            .filter(|&(q, _)| q != 0)
            .map(|(_, count)| count as u32)
            .product()
    }

    fn parse(input: &str, w: i32, h: i32) -> Self {
        fn parse_vec(s: &str) -> Vec2 {
            let (x, y) = s[2..].split_once(',').unwrap();
            Vec2::new(x.parse().unwrap(), y.parse().unwrap())
        }

        let robots = input
            .lines()
            .map(|s| {
                let (ps, vs) = s.split_once(' ').unwrap();

                Robot {
                    p: parse_vec(ps),
                    v: parse_vec(vs),
                }
            })
            .collect_vec();
        RobotSpace { w, h, robots }
    }
}

fn solve1(input: &str, w: i32, h: i32) -> u32 {
    let mut space = RobotSpace::parse(input, w, h);

    for _ in 0..100 {
        space.tick();
    }

    space.checksum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve1(input, 101, 103))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut space = RobotSpace::parse(input, 101, 103);

    for seconds in 0..u32::MAX {
        if space.has_tree() {
            return Some(seconds);
        }
        space.tick();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(solve1(&input, 11, 7), 12);
    }
}
