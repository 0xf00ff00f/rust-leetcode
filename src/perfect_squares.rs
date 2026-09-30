use crate::solution::Solution;

const MAX_N: i32 = 10_000;

// can't use std::cmp::min in const fn
const fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

const SOLUTIONS: [i32; MAX_N as usize + 1] = const {
    let mut solutions = [0; MAX_N as usize + 1];
    let mut i = 1;
    while i <= MAX_N {
        let mut best = i32::MAX;
        let mut p = 1;
        while p * p <= i {
            best = min(best, 1 + solutions[(i - p * p) as usize]);
            p += 1;
        }
        solutions[i as usize] = best;
        i += 1;
    }
    solutions
};

impl Solution {
    #[allow(dead_code)]
    pub fn num_squares(n: i32) -> i32 {
        assert!(n < SOLUTIONS.len() as i32);
        SOLUTIONS[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
