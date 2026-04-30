const MAX_N: usize = 46340; // sqrt(2**31 - 1)

const fn generate_squares() -> [i32; MAX_N + 1] {
    let mut squares = [0i32; MAX_N + 1];
    let mut i = 0;
    while i <= MAX_N as i32 {
        squares[i as usize] = i * i;
        i += 1;
    }
    squares
}

const SQUARES: [i32; MAX_N + 1] = generate_squares();

pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        SQUARES.binary_search(&num).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert!(Solution::is_perfect_square(0));
        assert!(Solution::is_perfect_square(16));
        assert!(!Solution::is_perfect_square(14));
        assert!(Solution::is_perfect_square(1073676289));
        assert!(Solution::is_perfect_square(2147395600));
    }
}
