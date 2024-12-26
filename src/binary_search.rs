/// Returns the first value of range for which the predicate returns true
pub fn binary_search<T>(range: std::ops::Range<T>, predicate: impl Fn(&T) -> bool) -> Option<T>
where
    T: Clone
        + Ord
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + From<u8>,
{
    let mut low = range.start;
    let mut high = range.end;
    let mut result: Option<T> = None;

    while low < high {
        let mid = low.clone() + ((high.clone() - low.clone()) / T::from(2));
        if predicate(&mid) {
            result = Some(mid.clone());
            high = mid.clone();
        } else {
            low = mid.clone() + T::from(1);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(0..6, |&x| x >= 5), Some(5));
        assert_eq!(binary_search(0..9, |&x| x >= 5), Some(5));
        assert_eq!(binary_search(0..10, |&x| x >= 5), Some(5));
        assert_eq!(binary_search(0..10, |&x| x >= 5), Some(5));
        assert_eq!(binary_search(0..11, |&x| x >= 5), Some(5));
        assert_eq!(binary_search(0..12, |&x| x >= 5), Some(5));

        assert_eq!(binary_search(0..0, |&x| x >= 5), None);
        assert_eq!(binary_search(0..5, |&x| x >= 5), None);
    }

    #[test]
    fn test_binary_search_finds_first_valid() {
        assert_eq!(binary_search(0..10, |&x| x >= 5), Some(5));
        assert_eq!(binary_search(0..15, |&x| x >= 7), Some(7));
        assert_eq!(binary_search(0..20, |&x| x >= 0), Some(0));
    }

    #[test]
    fn test_binary_search_no_valid_value() {
        assert_eq!(binary_search(0..10, |&x| x > 10), None);
        assert_eq!(binary_search(0..5, |&x| x < 0), None);
        assert_eq!(binary_search(5..10, |&x| x < 5), None);
    }

    #[test]
    fn test_binary_search_empty_range() {
        assert_eq!(binary_search(0..0, |&x| x >= 0), None);
        assert_eq!(binary_search(5..5, |&x| x >= 5), None);
    }

    #[test]
    fn test_binary_search_edge_cases() {
        assert_eq!(binary_search(0..1, |&x| x >= 0), Some(0));
        assert_eq!(binary_search(0..2, |&x| x >= 1), Some(1));
        assert_eq!(binary_search(10..20, |&x| x >= 15), Some(15));
    }
}
