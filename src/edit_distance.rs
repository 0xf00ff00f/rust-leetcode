use std::cmp::min;

struct Solution {}

impl Solution {
    #[allow(dead_code)]
    fn min_distance(s1_str: String, s2_str: String) -> i32 {
        let s1 = s1_str.as_bytes();
        let s2 = s2_str.as_bytes();
        let mut cache = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i1 in (0..=s1.len()).rev() {
            for i2 in (0..=s2.len()).rev() {
                cache[i1][i2] = if i1 == s1.len() && i2 == s2.len() {
                    0
                } else if i1 == s1.len() {
                    1 + cache[i1][i2 + 1]
                } else if i2 == s2.len() {
                    1 + cache[i1 + 1][i2]
                } else if s1[i1] == s2[i2] {
                    cache[i1 + 1][i2 + 1]
                } else {
                    1 + min(
                        min(cache[i1][i2 + 1], cache[i1 + 1][i2]),
                        cache[i1 + 1][i2 + 1],
                    )
                }
            }
        }
        cache[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
