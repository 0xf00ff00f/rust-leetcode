use crate::solution::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn combination_sum4(candidates: Vec<i32>, target: i32) -> i32 {
        let mut cache = vec![0; (target + 1) as usize];
        cache[0] = 1;
        for i in 1i32..=target {
            let mut r = 0;
            for &n in &candidates {
                if n <= i {
                    r += cache[(i - n) as usize];
                }
            }
            cache[i as usize] = r;
        }
        cache[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum4() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
    }
}
