use crate::solution::Solution;
use std::cmp;

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_delete_sum(s1_str: String, s2_str: String) -> i32 {
        let s1 = s1_str.as_bytes();
        let s2 = s2_str.as_bytes();
        let mut cache = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i1 in (0..=s1.len()).rev() {
            for i2 in (0..=s2.len()).rev() {
                cache[i1][i2] = if i1 == s1.len() && i2 == s2.len() {
                    0
                } else if i1 == s1.len() {
                    s2[i2] as i32 + cache[i1][i2 + 1]
                } else if i2 == s2.len() {
                    s1[i1] as i32 + cache[i1 + 1][i2]
                } else if s1[i1] == s2[i2] {
                    cache[i1 + 1][i2 + 1]
                } else {
                    cmp::min(
                        s2[i2] as i32 + cache[i1][i2 + 1],
                        s1[i1] as i32 + cache[i1 + 1][i2],
                    )
                }
            }
        }
        cache[0][0]
    }

    #[allow(dead_code)]
    pub fn minimum_delete_sum_naive(s1: String, s2: String) -> i32 {
        fn helper(s1: &[u8], s2: &[u8], i1: usize, i2: usize) -> i32 {
            if i1 == s1.len() && i2 == s2.len() {
                return 0;
            } else if i1 == s1.len() {
                return s2[i2] as i32 + helper(s1, s2, i1, i2 + 1);
            } else if i2 == s2.len() {
                return s1[i1] as i32 + helper(s1, s2, i1 + 1, i2);
            } else if s1[i1] == s2[i2] {
                return helper(s1, s2, i1 + 1, i2 + 1);
            } else {
                return cmp::min(
                    s2[i2] as i32 + helper(s1, s2, i1, i2 + 1),
                    s1[i1] as i32 + helper(s1, s2, i1 + 1, i2),
                );
            }
        }
        helper(s1.as_bytes(), s2.as_bytes(), 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_delete_sum() {
        assert_eq!(
            Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()),
            231
        );
        assert_eq!(
            Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()),
            403
        );
    }
}
