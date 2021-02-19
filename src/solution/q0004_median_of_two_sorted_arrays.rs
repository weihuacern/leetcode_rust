/**
 * [4] Median of Two Sorted Arrays
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 *
 * Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
 *
 * You may assume nums1 and nums2 cannot be both empty.
 *
 * Example 1:
 *
 *
 * nums1 = [1, 3]
 * nums2 = [2]
 *
 * The median is 2.0
 *
 *
 * Example 2:
 *
 *
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 *
 * The median is (2 + 3)/2 = 2.5
 *
 *
 */

use std::cmp;
use std::i32;

pub struct Solution {}

// Problem: https://leetcode.com/problems/median-of-two-sorted-arrays/

// Code start

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (n1, n2) = (nums1.len(), nums2.len());
        let n = n1 + n2;
        if n1 > n2 {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }
        if n1 == 0 {
            if (n2 % 2) != 0 {
                return nums2[n/2] as f64;
            } else {
                return (nums2[n/2-1] + nums2[n/2]) as f64 / 2.0;
            }
        }

        let (mut l_edge, mut r_edge) = (0, n1);
        let mut res: f64 = 0.0;
        while l_edge <= r_edge {
            let cur1 = l_edge + (r_edge - l_edge) / 2;
            let cur2 = (n + 1) / 2 - cur1;
            // Calculate l1, r1, l2, r2
            let l1 = if cur1 == 0 { i32::MIN } else { nums1[cur1 - 1] };
            let r1 = if cur1 == n1 { i32::MAX } else { nums1[cur1] };
            let l2 = if cur2 == 0 { i32::MIN } else { nums2[cur2 - 1] };
            let r2 = if cur2 == n2 { i32::MAX } else { nums2[cur2] };
            // Binary search
            if l1 > r2 {
                r_edge = cur1 - 1;
            } else if l2 > r1 {
                l_edge = cur1 + 1;
            } else {
                if (n % 2) != 0 {
                    res = cmp::max(l1, l2) as f64;
                } else {
                    res = (cmp::max(l1, l2) + cmp::min(r1, r2)) as f64 / 2.0;
                }
                break;
            }
        }
        res
    }
}

// Code end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0004() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![]),
            2.5
        );
    }
}
