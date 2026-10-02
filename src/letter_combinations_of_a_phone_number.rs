use std::collections::HashMap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn helper(digits: &[u8], index: usize, prefix: &mut String, results: &mut Vec<String>) {
            if index == digits.len() {
                results.push(prefix.clone());
            } else {
                // XXX this is horrible
                let digit_letters = HashMap::from([
                    (b'2', vec![b'a', b'b', b'c']),
                    (b'3', vec![b'd', b'e', b'f']),
                    (b'4', vec![b'g', b'h', b'i']),
                    (b'5', vec![b'j', b'k', b'l']),
                    (b'6', vec![b'm', b'n', b'o']),
                    (b'7', vec![b'p', b'q', b'r', b's']),
                    (b'8', vec![b't', b'u', b'v']),
                    (b'9', vec![b'w', b'x', b'y', b'z']),
                ]);
                if let Some(chars) = digit_letters.get(&digits[index]) {
                    for &c in chars {
                        prefix.push(c as char);
                        helper(digits, index + 1, prefix, results);
                        prefix.pop();
                    }
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
