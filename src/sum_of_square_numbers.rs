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

const MAX_PRIME: usize = 50_000; // should be sqrt(2**31 - 1)...
const PRIME_COUNT: usize = 5_133; // primes less than MAX_PRIME

const fn generate_primes() -> [i32; PRIME_COUNT] {
    let mut sieve = [true; MAX_PRIME];
    let mut p = 2;
    let mut primes = [0; PRIME_COUNT];
    let mut i: usize = 0;
    while p < sieve.len() {
        if sieve[p] {
            primes[i] = p as i32;
            let mut j: usize = p + p;
            while j < sieve.len() {
                sieve[j] = false;
                j += p;
            }
            i += 1;
        }
        p += 1;
    }
    assert!(i == PRIME_COUNT);
    primes
}

const PRIMES: [i32; PRIME_COUNT] = generate_primes();

pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn judge_square_sum(num: i32) -> bool {
        // https://en.wikipedia.org/wiki/Sum_of_two_squares_theorem
        let mut n = num;
        for p in PRIMES {
            if n <= 1 {
                break;
            }
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            if e % 2 != 0 && (p % 4) == 3 {
                return false;
            }
        }
        // if we're left with a prime check if it's (4n + 3)
        if n > 1 && (n % 4) == 3 {
            return false;
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
        assert!(!Solution::judge_square_sum(999999999));
        for i in 2..1000 {
            assert!(Solution::judge_square_sum_brute(i) == Solution::judge_square_sum(i));
        }
    }
}
