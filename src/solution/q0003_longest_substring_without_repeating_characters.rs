/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example:
 *
 * Input: "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 */

use std::collections::HashSet;
use std::cmp::max;

pub struct Solution {}

// Problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/

// Code start

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set: HashSet<char> = HashSet::new();
        let mut max_len = 0;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        for i in 0..n {
            for j in i..n {
                if !set.contains(&chars[j]) {
                    set.insert(chars[j]);
                } else {
                    max_len = max(max_len, set.len());
                    set.clear();
                    break;
                }
            }
        }
        max(max_len, set.len()) as i32
    }
}

// Code end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0003() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );

        assert_eq!(
            Solution::length_of_longest_substring("zzzz".to_string()),
            1
        );

        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );

        assert_eq!(
            Solution::length_of_longest_substring("".to_string()),
            0
        );
    }
}
