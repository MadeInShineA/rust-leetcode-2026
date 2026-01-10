// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/description/?envType=daily-question&envId=2026-01-02
// 961. N-Repeated Element in Size 2N Array
// Easy
// You are given an integer array nums with the following properties:
//  nums.length == 2 * n.
//  nums contains n + 1 unique elements.
//  Exactly one element of nums is repeated n times.
// Return the element that is repeated n times.

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut nums_set: HashSet<i32> = HashSet::new();

        for &element in nums.iter() {
            if !nums_set.insert(element) {
                return element;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_repeated_n_times() {
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
