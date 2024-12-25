use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(24);

struct Gate<'a> {
    lhs: &'a str,
    op: &'a str,
    rhs: &'a str,
    out: &'a str,
}

fn is_input_wire(w: &str) -> bool {
    w.starts_with('x') || w.starts_with('y')
}

fn is_output_wire(w: &str) -> bool {
    w.starts_with('z')
}

fn evaluate(gates: &HashMap<&str, Gate>, inputs: &HashMap<&str, bool>) -> u64 {

    fn evaluate_wire(wire: &str, gates: &HashMap<&str, Gate>, wire_values: &mut HashMap<&str, bool>) -> bool {
        if let Some(&value) = wire_values.get(wire) {
            value
        } else {
            let gate = gates.get(wire).unwrap();

            let lhs = evaluate_wire(gate.lhs, gates, wire_values);
            let rhs = evaluate_wire(gate.rhs, gates, wire_values);

            match gate.op {
                "AND" => lhs && rhs,
                "OR" => lhs || rhs,
                "XOR" => lhs ^ rhs,
                _ => panic!("invalid op {}", gate.op),
            }
        }
    }

    let mut wire_values = inputs.to_owned();
    let mut value = 0;

    for &z in gates.keys().filter(|&w| is_output_wire(w)).sorted().rev() {
        value <<= 1;
        if evaluate_wire(z, gates, &mut wire_values) {
            value |= 1;
        }
    }

    value
}

// The problem talks about detecting which pairs need to be swapped, but that's misleading,
// since the output is just a sorted list of wires. We don't even need to detect the pairs,
// just wires that are bad.
fn find_bad_wires<'a, 'b : 'a>(gates: &'b HashMap<&'a str, Gate>) -> Vec<&'a str> {
    fn is_wired_to_inputs(gate: &Gate) -> bool { is_input_wire(gate.lhs) && is_input_wire(gate.rhs) }

    let mut bad = Vec::new();

    for gate in gates.values() {
        let lhs = gates.get(gate.lhs);
        let rhs = gates.get(gate.rhs);
        let lhs_op = lhs.map(|g| g.op);
        let rhs_op = rhs.map(|g| g.op);

        if gate.out == "z01" || gate.out == "z45" {
            continue;
        }

        if is_output_wire(gate.out) {
            if gate.op != "XOR" {
                bad.push(gate.out);
            } else if lhs_op == Some("AND") {
                bad.push(gate.lhs);
            } else if rhs_op == Some("AND") {
                bad.push(gate.rhs);
            } else if rhs_op == Some("XOR") && lhs_op == Some("XOR") {
                if lhs.is_some() && !is_wired_to_inputs(lhs.unwrap()) {
                    bad.push(gate.lhs);
                } else if rhs.is_some() && !is_wired_to_inputs(rhs.unwrap()) {
                    bad.push(gate.rhs);
                }
            }
        } else if gate.op == "OR" {
            if lhs_op != Some("AND") {
                bad.push(gate.lhs);
            } else if rhs_op != Some("AND") {
                bad.push(gate.rhs);
            }
        }
    }

    bad
}

fn parse(input: &str) -> (HashMap<&str, Gate>, HashMap<&str, bool>) {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let mut hardwires = HashMap::new();
    let mut gates = HashMap::new();

    for p in p1.lines() {
        let (g, v) = p.split_once(": ").unwrap();
        hardwires.insert(g, v.parse::<u8>().unwrap() == 1);
    }

    for p in p2.lines() {
        let (lhs, op, rhs, _, out) = p.split(" ").collect_tuple().unwrap();
        gates.insert(out, Gate { lhs, op, rhs, out });
    }

    (gates, hardwires)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (circuit, hardwires) = parse(input);
    Some(evaluate(&circuit, &hardwires))
}

pub fn part_two(input: &str) -> Option<String> {
    let (circuit, _) = parse(input);
    Some(find_bad_wires(&circuit).iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(&input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_one(&input);
        assert_eq!(result, Some(2024));
    }
}
