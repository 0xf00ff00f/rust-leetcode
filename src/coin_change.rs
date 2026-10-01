struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut cache = vec![-1; (amount + 1) as usize];
        cache[0] = 0;
        for i in 1..=amount {
            let mut result = -1;
            for &c in &coins {
                if c <= i {
                    let t = cache[(i - c) as usize];
                    if t != -1 && (result == -1 || 1 + t < result) {
                        result = 1 + t;
                    }
                }
            }
            cache[i as usize] = result;
        }
        cache[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_replacement() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 1), 1);
        assert_eq!(Solution::coin_change(vec![2], 2), 1);
        assert_eq!(Solution::coin_change(vec![1, 2], 2), 1);
        assert_eq!(Solution::coin_change(vec![1, 2], 3), 2);
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }
}
