#![allow(dead_code)]

use std::collections::BTreeSet;

struct Solution;

impl Solution {
    // 数组法
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut ans = 0;
        let mut index = [-1; 128];
        let mut i = -1;

        for (j, c) in s.chars().enumerate() {
            i = index[c as usize].max(i);
            ans = ans.max((j as i32) - i);
            index[c as usize] = j as i32;
        }

        ans
    }

    // 滑动窗口法
    pub fn length_of_longest_substring_2(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut c_left = char::default();
        let mut r_index = 0;
        let mut max_len = 0;
        let mut set = BTreeSet::new();
        let s = s.chars();

        for (i1, c1) in s.clone().enumerate() {
            set.remove(&c_left);
            c_left = c1;

            for (_i2, c2) in s.clone().skip(r_index).enumerate() {
                if set.contains(&c2) {
                    break;
                }
                set.insert(c2);
                r_index += 1;
            }

            println!(
                "i1={} r_index={} c_left={} set={:?}",
                i1, r_index, c_left, set
            );
            max_len = max_len.max(set.len());
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn1() {
        let s = "abcabcbb".into();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 3);

        let s = "bbbb".into();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 1);

        let s = "pwwkew".into();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 3);

        let s = "".into();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 0);

        let s = " ".into();
        let len = Solution::length_of_longest_substring(s);
        assert_eq!(len, 1);
    }

    #[test]
    fn test_fn2() {
        let s = "abcabcbb".into();
        let len = Solution::length_of_longest_substring_2(s);
        assert_eq!(len, 3);

        let s = "bbbb".into();
        let len = Solution::length_of_longest_substring_2(s);
        assert_eq!(len, 1);

        let s = "pwwkew".into();
        let len = Solution::length_of_longest_substring_2(s);
        assert_eq!(len, 3);

        let s = "".into();
        let len = Solution::length_of_longest_substring_2(s);
        assert_eq!(len, 0);

        let s = " ".into();
        let len = Solution::length_of_longest_substring_2(s);
        assert_eq!(len, 1);
    }
}
