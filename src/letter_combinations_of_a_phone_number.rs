struct Solution;

pub const DIGIT_LETTERS: &[&[u8]] = &[
    &[b'a', b'b', b'c'],
    &[b'd', b'e', b'f'],
    &[b'g', b'h', b'i'],
    &[b'j', b'k', b'l'],
    &[b'm', b'n', b'o'],
    &[b'p', b'q', b'r', b's'],
    &[b't', b'u', b'v'],
    &[b'w', b'x', b'y', b'z'],
];

impl Solution {
    #[allow(dead_code)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn helper(digits: &[u8], index: usize, prefix: &mut String, results: &mut Vec<String>) {
            if index == digits.len() {
                results.push(prefix.clone());
            } else {
                // tee-hee
                let chars = DIGIT_LETTERS[(digits[index] - b'2') as usize];
                for &c in chars {
                    prefix.push(c as char);
                    helper(digits, index + 1, prefix, results);
                    prefix.pop();
                }
            }
        }
        let mut results = Vec::new();
        helper(digits.as_bytes(), 0, &mut String::new(), &mut results);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gest_generate_parenthesis() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
