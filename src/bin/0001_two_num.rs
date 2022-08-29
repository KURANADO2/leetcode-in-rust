// https://leetcode.cn/problems/two-sum/
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, item) in nums.iter().enumerate() {
            match map.get(&(target - item)) {
                None => {
                    map.insert(item, index);
                }
                Some(sub_index) => return vec![*sub_index as i32, index as i32],
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test test_0001 -- --show-output
    #[test]
    fn test_0001() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
