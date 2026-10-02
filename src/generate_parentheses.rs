struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn helper(left: i32, used: i32, prefix: &mut String, results: &mut Vec<String>) {
            if left == 0 && used == 0 {
                results.push(prefix.clone());
            } else {
                if left > 0 {
                    prefix.push('(');
                    helper(left - 1, used + 1, prefix, results);
                    prefix.pop();
                }
                if used > 0 {
                    prefix.push(')');
                    helper(left, used - 1, prefix, results);
                    prefix.pop();
                }
            }
        }
        let mut results = Vec::new();
        helper(n, 0, &mut String::new(), &mut results);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gest_generate_parenthesis() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
