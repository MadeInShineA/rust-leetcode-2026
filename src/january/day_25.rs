// https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/description/?envType=daily-question&envId=2026-01-25
// 1984. Minimum Difference Between Highest and Lowest of K Scores
// Easy
// You are given a 0-indexed integer array nums
// where nums[i] represents the score of the ith student.
// You are also given an integer k.
// Pick the scores of any k students from the array so that
// the difference between the highest and the lowest of the k scores is minimized.
// Return the minimum possible difference.

pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let k: usize = k as usize;
        nums.sort();

        let n: usize = nums.len();
        let mut min_diff: i32 = i32::MAX;

        for i in 0..(n - k + 1) {
            let diff: i32 = nums[i + k - 1] - nums[i];

            min_diff = min_diff.min(diff);
        }

        min_diff
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_difference() {
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }
}
