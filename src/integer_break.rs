use crate::solution::Solution;

const MAX_N: i32 = 58;

// can't use std::cmp::max in const fn
const fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

const SOLUTIONS: [i32; MAX_N as usize + 1] = const {
    let mut solutions = [0; MAX_N as usize + 1];
    let mut i = 2;
    while i <= MAX_N {
        let mut best = 0;
        let mut r = 1;
        // TODO: should be able to test just half of this range?
        while r < i {
            best = max(best, (i - r) * max(r, solutions[r as usize]));
            r += 1;
        }
        solutions[i as usize] = best;
        i += 1;
    }
    solutions
};

impl Solution {
    #[allow(dead_code)]
    pub fn integer_break(n: i32) -> i32 {
        assert!(n < SOLUTIONS.len() as i32);
        SOLUTIONS[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_break() {
        assert_eq!(Solution::integer_break(1), 0);
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(4), 4);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
