struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_tilings_naive(n: i32) -> i32 {
        fn helper(state: u32, remaining: i32) -> i32 {
            if remaining == 0 {
                if state == 0 {
                    1
                } else {
                    0
                }
            } else {
                match state {
                    0 => {
                        helper(0, remaining - 1)
                            + helper(1, remaining - 1)
                            + helper(2, remaining - 1)
                            + helper(3, remaining - 1)
                    }
                    1 => helper(2, remaining - 1) + helper(3, remaining - 1),
                    2 => helper(1, remaining - 1) + helper(3, remaining - 1),
                    3 => helper(0, remaining - 1),
                    _ => -1,
                }
            }
        }
        helper(0, n)
    }

    #[allow(dead_code)]
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut cache = [[1, 0, 0, 0], [0, 0, 0, 0]];
        for remaining in 1usize..=(n as usize) {
            let cur = remaining & 1;
            let prev = cur ^ 1;
            cache[cur][0] = (((cache[prev][0] + cache[prev][1]) % MOD)
                + ((cache[prev][2] + cache[prev][3]) % MOD))
                % MOD;
            cache[cur][1] = (cache[prev][2] + cache[prev][3]) % MOD;
            cache[cur][2] = (cache[prev][1] + cache[prev][3]) % MOD;
            cache[cur][3] = cache[prev][0];
        }
        cache[(n as usize) & 1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_tilings() {
        assert_eq!(Solution::num_tilings(1), 1);
        assert_eq!(Solution::num_tilings(2), 2);
        assert_eq!(Solution::num_tilings(3), 5);
        assert_eq!(Solution::num_tilings(30), 312342182);
    }
}
