use crate::solution::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut cache = vec![vec![0; n as usize]; m as usize];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mut count = 0;
                if i == m - 1 && j == n - 1 {
                    count = 1;
                } else {
                    if i < m - 1 {
                        count += cache[(i + 1) as usize][j as usize];
                    }
                    if j < n - 1 {
                        count += cache[i as usize][(j + 1) as usize];
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
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }
}
