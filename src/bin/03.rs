use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = re
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            x * y
        })
        .sum::<i32>();
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();

    let mut enabled = true;
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        if let Some(do_match) = cap.get(0) {
            match do_match.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        let x = cap[2].parse::<i32>().unwrap();
                        let y = cap[3].parse::<i32>().unwrap();
                        sum += x * y;
                    }
                }
            }
        }
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
