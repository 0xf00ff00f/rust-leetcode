use std::cmp::min;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn integer_replacement(n: i32) -> i32 {
        fn helper(n: u32, cache: &mut HashMap<u32, i32>) -> i32 {
            // NB: can't use cache.entry(...).or_insert(...) here, entry() borrows the map mutably
            if let Some(&result) = cache.get(&n) {
                result
            } else {
                let result = if n <= 1 {
                    0
                } else if n % 2 == 0 {
                    1 + helper(n / 2, cache)
                } else {
                    2 + min(helper((n + 1) / 2, cache), helper((n - 1) / 2, cache))
                };
                cache.insert(n, result);
                result
            }
        }
        let mut cache = HashMap::new();
        helper(n as u32, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_replacement() {
        assert_eq!(Solution::integer_replacement(8), 3);
        assert_eq!(Solution::integer_replacement(7), 4);
        assert_eq!(Solution::integer_replacement(4), 2);
    }
}
