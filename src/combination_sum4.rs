use crate::solution::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn combination_sum4(candidates: Vec<i32>, target: i32) -> i32 {
        let mut cache = vec![0; target as usize];
        for i in 1i32..=target {
            let mut r = 0;
            for j in 0..candidates.len() {
                let n = candidates[j];
                if n == i {
                    r += 1;
                } else if n < i {
                    r += cache[(i - n) as usize - 1];
                }
            }
            cache[i as usize - 1] = r;
        }
        cache[target as usize - 1] as i32
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
