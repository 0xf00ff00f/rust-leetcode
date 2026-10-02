struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn helper(left: i32, used: i32, prefix: String, results: &mut Vec<String>) {
            if left == 0 && used == 0 {
                results.push(prefix);
            } else {
                if left > 0 {
                    helper(left - 1, used + 1, format!("{}(", prefix), results);
                }
                if used > 0 {
                    helper(left, used - 1, format!("{})", prefix), results);
                }
            }
        }
        let mut results = Vec::new();
        helper(n, 0, String::new(), &mut results);
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
