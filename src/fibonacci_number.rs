struct Solution {}

const MAX_N: usize = 30;

const SOLUTIONS: [i32; MAX_N as usize + 1] = const {
    let mut solutions = [0; MAX_N as usize + 1];
    solutions[0] = 0;
    solutions[1] = 1;
    let mut i: usize = 2;
    while i <= MAX_N {
        solutions[i] = solutions[i - 1] + solutions[i - 2];
        i += 1;
    }
    solutions
};

impl Solution {
    #[allow(dead_code)]
    pub fn fib(n: i32) -> i32 {
        assert!(n < SOLUTIONS.len() as i32);
        SOLUTIONS[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}
