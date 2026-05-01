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
    pub fn judge_square_sum_brute(num: i32) -> bool {
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
    }

    pub fn judge_square_sum(num: i32) -> bool {
        // https://en.wikipedia.org/wiki/Sum_of_two_squares_theorem
        let mut n = num;
        if n == 0 {
            return true;
        }
        // get rid of powers of 2
        while (n % 2) == 0 {
            n /= 2
        }
        // TODO: need a table of pythagorean primes
        let mut p = 3;
        while n > 1 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            if e > 0 && (p % 4) == 3 {
                if e % 2 != 0 {
                    return false;
                }
            }
            p += 2;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_square_sum() {
        assert!(Solution::judge_square_sum(0));
        assert!(Solution::judge_square_sum(5));
        assert!(Solution::judge_square_sum(2));
        assert!(!Solution::judge_square_sum(3));
        assert!(!Solution::judge_square_sum(21));
        assert!(Solution::judge_square_sum(45));
        assert!(Solution::judge_square_sum(2450));
        for i in 2..1000 {
            assert!(Solution::judge_square_sum_brute(i) == Solution::judge_square_sum(i));
        }
    }
}
