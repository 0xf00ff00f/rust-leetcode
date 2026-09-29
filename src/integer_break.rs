use crate::solution::Solution;
use std::cmp;

impl Solution {
    #[allow(dead_code)]
    pub fn integer_break(n: i32) -> i32 {
        let mut cache = vec![0; (n + 1) as usize];
        cache[1] = 0;
        for i in 2..=n {
            let mut best = 0;
            for r in 1..i {
                best = cmp::max(best, (i - r) * cmp::max(r, cache[r as usize]));
            }
            cache[i as usize] = best;
        }
        cache[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_break() {
        assert_eq!(Solution::integer_break(1), 0);
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
