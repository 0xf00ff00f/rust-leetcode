use crate::solution::Solution;
use std::cmp;

impl Solution {
    #[allow(dead_code)]
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut cache = vec![vec![0; n as usize]; m as usize];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mut best = i32::MAX;
                if i == m - 1 && j == n - 1 {
                    best = grid[i][j];
                } else {
                    if i < m - 1 {
                        best = cmp::min(best, cache[(i + 1) as usize][j as usize]);
                    }
                    if j < n - 1 {
                        best = cmp::min(best, cache[i as usize][(j + 1) as usize]);
                    }
                    best += grid[i][j];
                }
                cache[i as usize][j as usize] = best;
            }
        }
        cache[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }
}
