pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn my_sqrt(a: i32) -> i32 {
        if a == 0 {
            return 0;
        }
        let mut x0 = a as u64;
        loop {
            let x1 = (x0 + (a as u64) / x0) >> 1;
            if x1 >= x0 {
                return x0 as i32;
            }
            x0 = x1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(3), 1);
        assert_eq!(Solution::my_sqrt(256), 16);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
        assert_eq!(Solution::my_sqrt(2147483647), 46340);
    }
}
