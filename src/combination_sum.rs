use crate::solution::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut cache = vec![vec![false; candidates.len()]; target as usize];
        for i in 1i32..=target {
            for j in 0..candidates.len() {
                let mut r = false;
                let n = candidates[j];
                if n <= i {
                    for k in j..candidates.len() {
                        if i - n == 0 || cache[(i - n) as usize - 1][k] {
                            r = true;
                            break;
                        }
                    }
                }
                cache[i as usize - 1][j] = r;
            }
        }
        fn enumerate_solutions(
            target: i32,
            from: usize,
            cache: &Vec<Vec<bool>>,
            candidates: &Vec<i32>,
        ) -> Vec<Vec<i32>> {
            assert!(target > 0);
            let mut results: Vec<Vec<i32>> = Vec::new();
            for i in from..candidates.len() {
                if cache[target as usize - 1][i] {
                    let n = candidates[i];
                    if n == target {
                        results.push(vec![n]);
                    } else {
                        let tails = enumerate_solutions(target - n, i, &cache, &candidates);
                        for mut tail in tails {
                            tail.push(n);
                            results.push(tail);
                        }
                    }
                }
            }
            results
        }
        enumerate_solutions(target, 0, &cache, &candidates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![3, 2, 2], vec![7]]
        );
    }
}
