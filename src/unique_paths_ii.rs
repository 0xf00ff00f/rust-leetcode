use crate::solution::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut cache = vec![vec![0; n as usize]; m as usize];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mut count = 0;
                if obstacle_grid[i][j] == 0 {
                    if i == m - 1 && j == n - 1 {
                        count = 1;
                    } else {
                        if i < m - 1 && obstacle_grid[i + 1][j] == 0 {
                            count += cache[(i + 1) as usize][j as usize];
                        }
                        if j < n - 1 && obstacle_grid[i][j + 1] == 0 {
                            count += cache[i as usize][(j + 1) as usize];
                        }
                    }
                }
                cache[i as usize][j as usize] = count;
            }
        }
        cache[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1, 0]]), 0);
    }
}
