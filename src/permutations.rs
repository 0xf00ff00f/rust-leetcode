struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn permute(mut n: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(index: usize, n: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
            if index == n.len() {
                results.push(n.clone());
            } else {
                for i in index..n.len() {
                    n.swap(index, i);
                    helper(index + 1, n, results);
                    n.swap(index, i);
                }
            }
        }
        let mut results = Vec::new();
        helper(0, &mut n, &mut results);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            [
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 2, 1],
                [3, 1, 2]
            ]
        );
    }
}
