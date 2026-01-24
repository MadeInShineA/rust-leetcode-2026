// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/description/?envType=daily-question&envId=2026-01-24
// 1877. Minimize Maximum Pair Sum in Array
// Medium
// The pair sum of a pair (a,b) is equal to a + b. The maximum pair sum is the largest pair sum in a list of pairs.
// Given an array nums of even length n, pair up the elements of nums into n / 2 pairs such that:
//  Each element of nums is in exactly one pair, and
//  The maximum pair sum is minimized.
// Return the minimized maximum pair sum after optimally pairing up the elements.

pub struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let n: usize = nums.len();

        let mut max_sum: i32 = 0;

        for i in 0..n / 2 {
            let sum: i32 = nums[i] + nums[n - 1 - i];

            max_sum = max_sum.max(sum);
        }
        max_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_pair_sum() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
    }
}
