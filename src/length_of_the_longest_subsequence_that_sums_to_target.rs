struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let mut cache = vec![vec![0; nums.len() + 1]; target as usize + 1];
        for i in 1..=target {
            cache[i as usize][nums.len()] = -1;
            for (j, &n) in nums.iter().enumerate().rev() {
                cache[i as usize][j] = if i == 0 {
                    0
                } else {
                    let mut r = cache[i as usize][j + 1];
                    if n <= i {
                        let t = cache[(i - n) as usize][j + 1];
                        if t != -1 && (r == -1 || t + 1 > r) {
                            r = t + 1;
                        }
                    }
                    r
                }
            }
        }
        cache[target as usize][0]
    }

    #[allow(dead_code)]
    pub fn length_of_longest_subsequence_naive(nums: Vec<i32>, target: i32) -> i32 {
        fn helper(nums: &Vec<i32>, target: i32, index: usize) -> i32 {
            let r = if target == 0 {
                0
            } else if index == nums.len() {
                -1
            } else {
                let mut r = -1;
                if nums[index] <= target {
                    let t = helper(nums, target - nums[index], index + 1);
                    if t != -1 {
                        r = t + 1;
                    }
                }
                let t = helper(nums, target, index + 1);
                if t != -1 && (r == -1 || t > r) {
                    r = t;
                }
                r
            };
            r
        }
        helper(&nums, target, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(Solution::length_of_longest_subsequence(vec![1], 1), 1);
        assert_eq!(Solution::length_of_longest_subsequence(vec![1, 2], 3), 2);
        assert_eq!(Solution::length_of_longest_subsequence(vec![1, 2, 3], 3), 2);
        assert_eq!(Solution::length_of_longest_subsequence(vec![2, 3], 5), 2);
        assert_eq!(Solution::length_of_longest_subsequence(vec![1, 2, 3], 5), 2);
        assert_eq!(Solution::length_of_longest_subsequence(vec![1, 2, 3, 4, 5], 9), 3);
        assert_eq!(Solution::length_of_longest_subsequence(vec![4, 1, 3, 2, 1, 5], 7), 4);
    }
}
