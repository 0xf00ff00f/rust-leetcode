const fn generate_squares() -> [u32; 1 << 16] {
    let mut table = [0u32; 1 << 16];
    let mut i = 0u32;
    while i < (1 << 16) {
        table[i as usize] = i * i;
        i += 1;
    }
    table
}

const SQUARES: [u32; 1 << 16] = generate_squares();

pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        SQUARES.binary_search(&(num as u32)).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(Solution::is_perfect_square(0));
        assert!(Solution::is_perfect_square(16));
        assert!(!Solution::is_perfect_square(14));
        assert!(Solution::is_perfect_square(1073676289));
        assert!(Solution::is_perfect_square(2147395600));
    }
}
