struct Solution;

pub const DIGIT_LETTERS: &[&[char]] = &[
    &['a', 'b', 'c'],
    &['d', 'e', 'f'],
    &['g', 'h', 'i'],
    &['j', 'k', 'l'],
    &['m', 'n', 'o'],
    &['p', 'q', 'r', 's'],
    &['t', 'u', 'v'],
    &['w', 'x', 'y', 'z'],
];

impl Solution {
    #[allow(dead_code)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn helper<I>(mut digits: I, prefix: &mut String, results: &mut Vec<String>)
        where
            I: Iterator<Item = char> + Clone,
        {
            match digits.next() {
                None => results.push(prefix.clone()),
                Some(digit) => {
                    let chars = DIGIT_LETTERS[digit as usize - '2' as usize];
                    for &c in chars {
                        prefix.push(c);
                        helper(digits.clone(), prefix, results);
                        prefix.pop();
                    }
                }
            }
        }
        let mut results = Vec::new();
        helper(digits.chars(), &mut String::new(), &mut results);
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
