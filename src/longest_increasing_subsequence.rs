struct Solution;

impl Solution {
    // TODO: O(n log n), see https://en.wikipedia.org/wiki/Longest_increasing_subsequence
    #[allow(dead_code)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}
