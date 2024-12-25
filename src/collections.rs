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

pub fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Ensure all rows have the same length
    assert!(matrix.iter().all(|row| row.len() == cols));

    (0..cols)
        .map(|i| (0..rows).map(|j| matrix[j][i].clone()).collect())
        .collect()
}
