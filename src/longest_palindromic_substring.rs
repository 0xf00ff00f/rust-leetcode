pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> String {
        let mut cache = vec![vec![false; s.len()]; s.len()];
        let chars = s.as_bytes();
        let mut longest = &chars[0..0];
        for i in 0..s.len() {
            for j in 0..s.len() - i {
                let is_palindrome = match i {
                    0 => true,
                    1 => chars[j] == chars[j + 1],
                    _ => chars[j] == chars[j + i] && cache[i - 2][j + 1],
                };
                if is_palindrome && i + 1 > longest.len() {
                    longest = &chars[j..j + i + 1];
                }
                cache[i][j] = is_palindrome;
            }
        }
        String::from_utf8(longest.to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::longest_palindrome("".to_string()).len(), 0);
        assert_eq!(Solution::longest_palindrome("xyz".to_string()).len(), 1);
        assert_eq!(Solution::longest_palindrome("babad".to_string()).len(), 3);
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()).len(), 2);
    }
}
