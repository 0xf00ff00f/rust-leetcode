use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = HashMap::new();
        let mut best_length = 0;
        let mut start = -1;
        for (i, c) in s.chars().enumerate() {
            if let Some(&last) = last_seen.get(&c) {
                start = max(start, last);
            }
            last_seen.insert(c, i as i32);
            best_length = max(i as i32 - start, best_length);
        }
        best_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
