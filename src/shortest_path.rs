use hashbrown::{HashMap, HashSet};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::hash::Hash;

pub trait Graph {
    type Node: Eq + Hash + Clone;

    fn is_solution(&self, node: &Self::Node) -> bool;
    fn collect_neighbors(&self, node: &Self::Node, neighbors: &mut Vec<(Self::Node, u32)>);
    fn heuristic_distance(&self, _node: &Self::Node) -> u32 {
        0
    }
}

pub fn shortest_path_len<G: Graph>(g: &G, start: G::Node) -> Option<(G::Node, u32)> {
    let mut g_score = HashMap::<G::Node, u32>::new();
    let mut open_set = PriorityQueue::<G::Node, Reverse<u32>>::new();
    let mut neighbors = Vec::new();

    g_score.insert(start.clone(), 0);
    let start_distance = g.heuristic_distance(&start);
    open_set.push(start, Reverse(start_distance));

    while let Some((current, _)) = open_set.pop() {
        let current_gscore = *g_score.get(&current).unwrap();

        if g.is_solution(&current) {
            return Some((current, current_gscore));
        }

        g.collect_neighbors(&current, &mut neighbors);
        for (neighbor, cost) in neighbors.drain(..) {
            let tentative_gscore = current_gscore + cost;
            if tentative_gscore < g_score.get(&neighbor).copied().unwrap_or(u32::MAX) {
                g_score.insert(neighbor.clone(), tentative_gscore);

                let neighbor_score = tentative_gscore + g.heuristic_distance(&neighbor);
                open_set.push(neighbor, Reverse(neighbor_score));
            }
        }
    }

    None
}

pub fn nodes_on_all_shortest_paths<G: Graph>(g: &G, start: G::Node) -> (u32, HashSet<G::Node>) {
    let initial = PathNode {
        point: start,
        total_cost: 0,
    };
    let mut queue = PriorityQueue::<PathNode<G::Node>, Reverse<u32>>::new();
    queue.push(initial, Reverse(0));

    let mut costs = HashMap::<G::Node, u32>::new();
    let mut best = u32::MAX;
    let mut neighbors = Vec::new();
    let mut previous = HashMap::<G::Node, Vec<G::Node>>::new();

    let mut final_states = Vec::new();

    while let Some((u, _)) = queue.pop() {
        let total_cost = u.total_cost;

        if total_cost > best {
            continue;
        }

        if g.is_solution(&u.point) {
            best = total_cost;
            final_states.push(u.point);
        } else {

            g.collect_neighbors(&u.point, &mut neighbors);
            for (v, cost) in neighbors.drain(..) {
                let new_cost = total_cost + cost;

                let seen_cost = costs.get(&v).copied().unwrap_or(u32::MAX);
                if new_cost < seen_cost {
                    costs.insert(v.clone(), new_cost);
                    queue.push(u.extend(v.clone(), new_cost), Reverse(new_cost));
                    previous.entry(v).or_default().push(u.point.clone());
                } else if new_cost == seen_cost {
                    previous
                        .get_mut(&v)
                        .expect("no previous")
                        .push(u.point.clone());
                }
            }
        }
    }

    let mut result = HashSet::<G::Node>::new();
    while let Some(value) = final_states.pop() {
        if result.insert(value.clone()) {
            if let Some(previous_states) = previous.get(&value) {
                for previous in previous_states {
                    final_states.push(previous.clone())
                }
            }
        }
    }

    (best, result)
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct PathNode<T> {
    point: T,
    total_cost: u32,
}

impl<T: Clone> PathNode<T> {
    fn extend(&self, point: T, total_cost: u32) -> PathNode<T> {
        PathNode { point, total_cost }
    }
}
