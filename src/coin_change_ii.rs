struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut cache = vec![vec![0; coins.len()]; amount as usize + 1];
        cache[0].fill(1);
        for i in 1i32..=amount {
            for j in (0..coins.len()).rev() {
                let mut r = 0;
                let n = coins[j];
                if n <= i {
                    r += cache[(i - n) as usize][j];
                }
                if j < coins.len() - 1 {
                    r += cache[i as usize][j + 1];
                }
                cache[i as usize][j] = r;
            }
        }
        cache[amount as usize][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(Solution::change(1, vec![1]), 1);
        assert_eq!(Solution::change(1, vec![2]), 0);
        assert_eq!(Solution::change(2, vec![1]), 1);
        assert_eq!(Solution::change(0, vec![1]), 1);
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    }
}
