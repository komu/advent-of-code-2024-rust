use advent_of_code::shortest_path::{shortest_path_len, Graph};
use hashbrown::HashMap;
use std::hash::Hash;
use strum_macros::{EnumIter, EnumString};
use strum::IntoEnumIterator;
use std::str::FromStr;

advent_of_code::solution!(21);

#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Hash, EnumString)]
enum Num { N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, NA }

#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Hash, EnumString, EnumIter)]
enum Arrow { AL, AU, AD, AR, AA }

/// Transition on T, performing a single lower level action A
type Transition<T, A> = (T, A);

/// Valid transitions between elements of T, actions being elements of A
struct Transitions<T, A>(HashMap<T, Vec<Transition<T, A>>>);

impl<T: Eq + Hash, A> Transitions<T, A> {
    fn get(&self, c: T) -> &[Transition<T, A>] {
        self.0.get(&c).unwrap()
    }
}

fn arrow_transitions() -> Transitions<Arrow, Arrow> {
    use Arrow::*;
    let mut transitions = HashMap::new();
    transitions.insert(AA, vec![(AU, AL), (AR, AD)]);
    transitions.insert(AU, vec![(AA, AR), (AD, AD)]);
    transitions.insert(AL, vec![(AD, AR)]);
    transitions.insert(AD, vec![(AL, AL), (AU, AU), (AR, AR)]);
    transitions.insert(AR, vec![(AD, AL), (AA, AU)]);

    Transitions(transitions)
}

fn numpad_transitions() -> Transitions<Num, Arrow> {
    use Arrow::*;
    use Num::*;
    let mut transitions = HashMap::new();

    transitions.insert(NA, vec![(N0, AL), (N3, AU)]);
    transitions.insert(N0, vec![(NA, AR), (N2, AU)]);
    transitions.insert(N1, vec![(N4, AU), (N2, AR)]);
    transitions.insert(N2, vec![(N1, AL), (N5, AU), (N3, AR), (N0, AD)]);
    transitions.insert(N3, vec![(N2, AL), (N6, AU), (NA, AD)]);
    transitions.insert(N4, vec![(N7, AU), (N5, AR), (N1, AD)]);
    transitions.insert(N5, vec![(N4, AL), (N8, AU), (N6, AR), (N2, AD)]);
    transitions.insert(N6, vec![(N5, AL), (N9, AU), (N3, AD)]);
    transitions.insert(N7, vec![(N8, AR), (N4, AD)]);
    transitions.insert(N8, vec![(N7, AL), (N9, AR), (N5, AD)]);
    transitions.insert(N9, vec![(N8, AL), (N6, AD)]);

    Transitions(transitions)
}

struct Costs<T>(HashMap<(T, T), u64>);

impl<T: Eq + Hash + Copy> Costs<T> {
    fn press_cost(&self, a: T, b: T) -> u64 {
        *self.0.get(&(a, b)).unwrap()
    }

    fn next<U: Eq + Hash + Copy>(&self, transitions: &Transitions<U, T>, button_a: &T) -> Costs<U> {
        let mut new_costs = HashMap::new();

        for &a in transitions.0.keys() {
            for &b in transitions.0.keys() {
                new_costs.insert(
                    (a, b),
                    self.calculate_press_cost(&a, &b, transitions, button_a),
                );
            }
        }

        Costs(new_costs)
    }

    fn calculate_press_cost<U: Eq + Hash + Copy>(
        &self,
        start: &U,
        button_to_press: &U,
        transitions: &Transitions<U, T>,
        button_a: &T,
    ) -> u64 {
        let initial_state = PathState {
            location: *start,
            last_action: *button_a,
            pressed: false,
        };

        let graph: MyGraph<'_, T, U> = MyGraph {
            button_to_press,
            transitions,
            button_a,
            costs: self
        };
        shortest_path_len(&graph, initial_state).unwrap().1
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct PathState<T, U> {
    location: U,
    last_action: T,
    pressed: bool,
}

struct MyGraph<'a, T, U> {
    button_to_press: &'a U,
    button_a: &'a T,
    transitions: &'a Transitions<U, T>,
    costs: &'a Costs<T>,
}

impl<'a, T: Copy + Eq + Hash, U: Copy + Eq + Hash> Graph for MyGraph<'a, T, U> {
    type Node = PathState<T, U>;

    fn is_solution(&self, node: &Self::Node) -> bool {
        node.pressed
    }

    fn collect_neighbors(&self, node: &Self::Node, neighbors: &mut Vec<(Self::Node, u64)>) {
        if node.location == *self.button_to_press {
            neighbors.push((
                PathState {
                    location: node.location,
                    last_action: node.last_action,
                    pressed: true,
                },
                self.costs.press_cost(node.last_action, *self.button_a),
            ));
        } else {
            for &(target, action) in self.transitions.get(node.location) {
                neighbors.push((
                    PathState {
                        location: target,
                        last_action: action,
                        pressed: false,
                    },
                    self.costs.press_cost(node.last_action, action),
                ))
            }
        }
    }
}

fn initial_costs() -> Costs<Arrow> {
    let mut press_costs = HashMap::new();
    for a in Arrow::iter() {
        for b in Arrow::iter() {
            press_costs.insert((a, b), 1);
        }
    }
    Costs(press_costs)
}

fn create_costs(steps: u32) -> Costs<Num> {
    let mut costs = initial_costs();
    let arrow_transitions = arrow_transitions();
    let numpad_transitions = numpad_transitions();
    for _ in 0..steps {
        costs = costs.next(&arrow_transitions, &Arrow::AA)
    }

    costs.next(&numpad_transitions, &Arrow::AA)
}

fn complexity(code: &str, steps: u32) -> u64 {
    let costs = create_costs(steps);
    let mut previous = Num::NA;
    let mut path_length = 0;

    for c in code.chars() {
        let button = Num::from_str(&format!("N{c}")).unwrap();
        path_length += costs.press_cost(previous, button);
        previous = button;
    }

    code[..3].parse::<u64>().unwrap() * path_length
}

fn solve(input: &str, steps: u32) -> u64 {
    input.lines().map(|s| complexity(s, steps)).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
