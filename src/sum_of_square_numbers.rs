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
    pub fn judge_square_sum(num: i32) -> bool {
        fn is_perfect_square(num: i32) -> bool {
            SQUARES.binary_search(&num).is_ok()
        }
        for s in SQUARES {
            if s > num / 2 {
                break;
            }
            if is_perfect_square(num - s) {
                return true;
            }
        }
        false
        // TODO: should be using this instead:
        // https://en.wikipedia.org/wiki/Fermat%27s_theorem_on_sums_of_two_squares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_square_sum() {
        assert!(Solution::judge_square_sum(0));
        assert!(Solution::judge_square_sum(5));
        assert!(!Solution::judge_square_sum(3));
    }
}
