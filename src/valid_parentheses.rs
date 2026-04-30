pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                '(' | '[' | '{' => stack.push(c),
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(Solution::is_valid("".to_string()));
        assert!(Solution::is_valid("()".to_string()));
        assert!(Solution::is_valid("()[]{}".to_string()));
        assert!(!Solution::is_valid("(]".to_string()));
        assert!(Solution::is_valid("([])".to_string()));
        assert!(!Solution::is_valid("([)]".to_string()));
    }
}
