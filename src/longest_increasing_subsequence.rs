struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_lis_quadratic(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cache = vec![1; n];
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    cache[i] = cache[i].max(cache[j] + 1);
                }
            }
        }
        cache.into_iter().max().unwrap()
    }

    // O(n log n) solution, see https://cp-algorithms.com/dynamic_programming/longest_increasing_subsequence.html
    #[allow(dead_code)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub = Vec::new();
        for &n in &nums {
            if let Err(idx) = sub.binary_search(&n) {
                if idx == sub.len() {
                    sub.push(n);
                } else {
                    sub[idx] = n;
                }
            }
        }
        sub.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}
