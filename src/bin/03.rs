advent_of_code::solution!(3);

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Eq, PartialEq, Copy, Clone)]
#[rustfmt::skip]
enum State {
    E, M, MU, MUL, MULP, NUM1, COMMA, NUM2, D, DO, DOP, DON, DON_, DON_T, DON_TP,
}

fn value(b: u8) -> u32 {
    (b - b'0') as u32
}

fn solve(input: &str, part2: bool) -> u32 {
    use State::*;

    let mut num1: u32 = 0;
    let mut num2: u32 = 0;
    let mut sum: u32 = 0;

    let mut enabled = true;
    let mut state = E;

    for b in input.bytes() {
        macro_rules! reset {
            () => { if enabled && b == b'm' { M } else if part2 && b == b'd' { D } else { E } };
        }

        macro_rules! expect {
            ($expected:expr, $next:ident) => {
                if b == $expected { $next } else { reset!() }
            }
        }

        macro_rules! expect_first_digit {
            ($next:ident, $var:ident) => {
                if b.is_ascii_digit() { $var = value(b); $next } else { reset!() }
            };
        }

        macro_rules! expect_next_digit {
            ($next:ident, $var:ident, $after:expr, $after_state:expr) => {
                if b.is_ascii_digit() { $var = $var * 10 + value(b); $next } else if b == $after { $after_state } else { reset!() }
            };
        }

        state = match state {
            E => reset!(),
            M => expect!(b'u', MU),
            MU => expect!(b'l', MUL),
            MUL => expect!(b'(', MULP),
            MULP => expect_first_digit!(NUM1, num1),
            NUM1 => expect_next_digit!(NUM1, num1, b',', COMMA),
            COMMA => expect_first_digit!(NUM2, num2),
            NUM2 => expect_next_digit!(NUM2, num2, b')', { sum += num1 * num2; E} ),
            D => expect!(b'o', DO),
            DO => if b == b'n' { DON } else if b == b'(' { DOP } else { reset!() },
            DOP => if b == b')' { enabled = true; E } else { reset!() },
            DON => expect!(b'\'', DON_),
            DON_ => expect!(b't', DON_T),
            DON_T => expect!(b'(', DON_TP),
            DON_TP => if b == b')' { enabled = false; E } else { reset!() },
        };
    }
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

#[rustfmt::skip]
pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
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
