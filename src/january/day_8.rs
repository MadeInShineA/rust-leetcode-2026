// https://leetcode.com/problems/max-dot-product-of-two-subsequences/description/?envType=daily-question&envId=2026-01-08
// 1458. Max Dot Product of Two Subsequences
// Hard
// Given two arrays nums1 and nums2.
// Return the maximum dot product between non-empty subsequences of nums1 and nums2 with the same length.
// A subsequence of a array is a new array which is formed from the original array by deleting some
// (can be none) of the characters without disturbing the relative positions of the remaining characters.
// (ie, [2,3,5] is a subsequence of [1,2,3,4,5] while [1,5,3] is not).

/*
use std::{
    iter::Sum,
    ops::{Add, Mul},
};
*/

use std::cmp::max;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        /*
        fn dot<T>(vec1: &[T], vec2: &[T]) -> T
        where
            T: Add<Output = T> + Mul<Output = T> + Copy + Sum,
        {
            vec1.iter()
                .zip(vec2.iter())
                .map(|(&vec1_element, &vec2_element)| vec1_element * vec2_element)
                .sum()
        }

        fn max_dot_product_rec(
            nums1_current_idx: usize,
            nums2_current_idx: usize,
            nums1_subsequence: &Vec<i32>,
            nums2_subsequence: &Vec<i32>,
            current_dot_product_max: i32,
            nums1: &Vec<i32>,
            nums2: &Vec<i32>,
        ) -> i32 {
            if nums1_current_idx > nums1.len() && nums2_current_idx > nums2.len() {
                current_dot_product_max
            } else {
                let nums_1_element: i32 = nums1[nums1_current_idx];
                let nums_2_element: i32 = nums2[nums1_current_idx];

                // If we take both nums elements
                let mut new_nums1_subsequence: Vec<i32> = nums1_subsequence.clone();
                new_nums1_subsequence.push(nums_1_element);

                let mut new_nums2_subsequence: Vec<i32> = nums2_subsequence.clone();
                new_nums2_subsequence.push(nums_2_element);

                let current_dot: i32 = dot(&new_nums1_subsequence, &new_nums2_subsequence);

                let max_dot: i32 = i32::max(current_dot_product_max, current_dot);

                let take_take_value: i32 = max_dot_product_rec(
                    nums1_current_idx + 1,
                    nums2_current_idx + 1,
                    &new_nums1_subsequence,
                    &new_nums2_subsequence,
                    max_dot,
                    nums1,
                    nums2,
                );

                let take_not_take_value: i32 = max_dot_product_rec(
                    nums1_current_idx + 1,
                    nums2_current_idx + 1,
                    &new_nums1_subsequence,
                    nums2_subsequence,
                    max_dot,
                    nums1,
                    nums2,
                );

                let not_take_take_value: i32 = max_dot_product_rec(
                    nums1_current_idx + 1,
                    nums2_current_idx + 1,
                    nums1_subsequence,
                    &new_nums2_subsequence,
                    max_dot,
                    nums1,
                    nums2,
                );

                let not_take_not_take_value: i32 = max_dot_product_rec(
                    nums1_current_idx + 1,
                    nums2_current_idx + 1,
                    nums1_subsequence,
                    nums2_subsequence,
                    max_dot,
                    nums1,
                    nums2,
                );

                i32::max(
                    i32::max(take_take_value, take_not_take_value),
                    i32::max(not_take_take_value, not_take_not_take_value),
                )
            }
            // based on current idxs calculate dot product if taken and update max
            // if not taken advance in only 1 sub at a time or both at same time
        }
        max_dot_product_rec(0, 0, &Vec::new(), &Vec::new(), i32::MIN, &nums1, &nums2)
        */
        fn dfs(
            i: usize,
            j: usize,
            nums1: &[i32],
            nums2: &[i32],
            memo_map: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if i >= nums1.len() || j >= nums2.len() {
                return i32::MIN;
            }

            if let Some(&memo_element) = memo_map.get(&(i, j)) {
                return memo_element;
            }

            let product: i32 = nums1[i] * nums2[j];

            // Start with at least this pair
            let mut result = product;

            // Option 1: take this pair AND extend with a valid future subsequence
            let future = dfs(i + 1, j + 1, nums1, nums2, memo_map);
            if future != i32::MIN {
                result = max(result, product + future);
            }

            // Option 2: skip nums1[i]
            let skip_i = dfs(i + 1, j, nums1, nums2, memo_map);
            if skip_i != i32::MIN {
                result = max(result, skip_i);
            }

            // Option 3: skip nums2[j]
            let skip_j = dfs(i, j + 1, nums1, nums2, memo_map);
            if skip_j != i32::MIN {
                result = max(result, skip_j);
            }

            memo_map.insert((i, j), result);
            result
        }

        dfs(0, 0, &nums1, &nums2, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_max_dot_product() {
        assert_eq!(
            Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
            18
        );
        assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
        assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
    }
}
