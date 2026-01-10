// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/description/?envType=daily-question&envId=2026-01-10
// 712. Minimum ASCII Delete Sum for Two Strings
// Given two strings s1 and s2, return the lowest ASCII sum of deleted characters to make two strings equal.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        fn dfs(
            i: usize,
            j: usize,
            s1: &[u8],
            s2: &[u8],
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if let Some(&cached) = memo.get(&(i, j)) {
                cached
            } else {
                // s1[i:] and s2[j:] are empty
                let result: i32 = if i == s1.len() && j == s2.len() {
                    0
                // s1[i:] is empty
                } else if i == s1.len() {
                    s2[j..].iter().map(|&element| element as i32).sum()
                // s2[j:] is empty
                } else if j == s2.len() {
                    s1[i..].iter().map(|&element| element as i32).sum()
                // s1[i] == s2[j]
                } else if s1[i] == s2[j] {
                    dfs(i + 1, j + 1, s1, s2, memo)
                } else {
                    i32::min(
                        s1[i] as i32 + dfs(i + 1, j, s1, s2, memo),
                        s2[j] as i32 + dfs(i, j + 1, s1, s2, memo),
                    )
                };

                memo.insert((i, j), result);
                result
            }
        }
        dfs(
            0,
            0,
            &s1.into_bytes(),
            &s2.into_bytes(),
            &mut HashMap::new(),
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_minimum_delete_sum() {
        assert_eq!(
            Solution::minimum_delete_sum(String::from("sea"), String::from("eat")),
            231
        );
        assert_eq!(
            Solution::minimum_delete_sum(String::from("delete"), String::from("leet")),
            403
        );
    }
}
