/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they
 * add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 */
use std::collections::HashMap;

pub struct Solution {}

// Problem: https://leetcode.com/problems/two-sum/

// Code start

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                None => {
                    map.insert(num, i);
                }
                Some(sub_i) => {
                    return vec![*sub_i as i32, i as i32];
                }
            }
        }
        vec![]
    }
}

// Code end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0001() {
        assert_eq!(vec![1, 3], Solution::two_sum(vec![1, 3, 7, 6], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![11, 2, 4], 6));
    }
}
