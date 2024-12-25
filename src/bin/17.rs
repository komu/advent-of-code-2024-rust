use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug)]
struct VM<'a> {
    a: u64,
    b: u64,
    c: u64,
    program: &'a [u32],
    ip: usize,
    output: Vec<u32>,
}

impl VM<'_> {
    fn new(program: &[u32], a: u64, b: u64, c: u64) -> VM {
        VM {
            a,
            b,
            c,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn is_running(&self) -> bool {
        self.ip < self.program.len()
    }

    fn run_until_next_output(&mut self) {
        while self.is_running() {
            let inst = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            self.ip += 2;

            match inst {
                0 => self.a /= 1 << self.combo(operand),
                1 => self.b ^= operand as u64,
                2 => self.b = self.combo(operand) % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = operand as usize
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    self.output.push((self.combo(operand) % 8) as u32);
                    return;
                }
                6 => self.b = self.a / (1 << self.combo(operand)),
                7 => self.c = self.a / (1 << self.combo(operand)),
                _ => panic!("invalid {inst}"),
            }
        }
    }

    fn combo(&self, v: u32) -> u64 {
        match v {
            0..=3 => v as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo operand {v}"),
        }
    }
}

fn parse(input: &str) -> (Vec<u32>, u64, u64, u64) {
    let mut lines = input.lines();
    let a = lines.next().expect("a")[12..].parse::<u64>().unwrap();
    let b = lines.next().expect("b")[12..].parse::<u64>().unwrap();
    let c = lines.next().expect("c")[12..].parse::<u64>().unwrap();
    lines.next().expect("empty line");
    let program = lines.next().unwrap()[9..]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    (program, a, b, c)
}

pub fn part_one(input: &str) -> Option<String> {
    let (program, a, b, c) = parse(input);
    let mut vm = VM::new(&program, a, b, c);

    while vm.is_running() {
        vm.run_until_next_output();
    }

    Some(vm.output.iter().map(|x| x.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (program, a, b, c) = parse(input);

    let program = program.clone();
    let mut vm = VM::new(&program, a, b, c);

    Some(program.iter().rfold(0_u64, |a, &op| {
        let start = 8 * a;
        let end = start + 256;
        (start..end)
            .find(|&candidate| {
                vm.a = candidate;
                vm.ip = 0;
                vm.run_until_next_output();
                vm.output.pop().unwrap() == op
            })
            .unwrap()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(&input);
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two_1() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_two(&input);
        assert_eq!(result, Some(29328));
    }

    #[test]
    fn test_part_two_2() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(&input);
        assert_eq!(result, Some(117440));
    }
}
