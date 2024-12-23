use hashbrown::HashSet;
use std::hash::Hash;

pub fn extend<T : Eq + Hash + Clone, S : Clone + Extend<T>>(a: &S, b: T) -> S {
    let mut result = a.clone();
    result.extend([b]);
    result
}

pub fn intersection<T : Eq + Hash + Clone>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.intersection(b).cloned().collect()
}
