/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
 *
 * Example 1:
 *
 *
 * Input: "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: "cbbd"
 * Output: "bb"
 *
 *
 */
pub struct Solution {}

// Problem: https://leetcode.com/problems/longest-palindromic-substring/

// Code start

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        if len < 1 {
            return s;
        }

        let (mut idx, mut curr_len, mut curr_s, mut curr_e) = (0, 0, 0, 0);

        while idx < len {
            let (mut i, mut j) = (idx, idx);
            let ch = seq[idx];
            while i > 0 && seq[i - 1] == ch {
                i -= 1
            }
            while j < len - 1 && seq[j + 1] == ch {
                j += 1
            }
            idx = j + 1;
            while i > 0 && j < len - 1 && seq[i - 1] == seq[j + 1] {
                i -= 1;
                j += 1;
            }
            let max_len = j - i + 1;
            if max_len > curr_len {
                curr_len = max_len;
                curr_s = i;
                curr_e = j;
            }
            if max_len >= len - 1 {
                break;
            }
        }

        s[curr_s..curr_e + 1].to_string()
    }
}

// Code end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0005() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_string()), "aaaaa");

        assert_eq!(Solution::longest_palindrome("babab".to_string()), "babab");

        assert_eq!(Solution::longest_palindrome("babcd".to_string()), "bab");

        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");

        assert_eq!(Solution::longest_palindrome("bb".to_string()), "bb");

        assert_eq!(Solution::longest_palindrome("".to_string()), "");
    }
}
