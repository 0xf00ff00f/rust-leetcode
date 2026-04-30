use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for i in 0..nums.len() {
            let missing = target - nums[i];
            match seen.get(&missing) {
                Some(&number) => return vec![number as i32, i as i32],
                _ => {
                    seen.insert(nums[i], i as i32);
                }
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
