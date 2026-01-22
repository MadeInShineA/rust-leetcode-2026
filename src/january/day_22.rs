// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i/?envType=daily-question&envId=2026-01-22
// 3507. Minimum Pair Removal to Sort Array I
// Easy
// Given an array nums, you can perform the following operation any number of times:
//  Select the adjacent pair with the minimum sum in nums. If multiple such pairs exist, choose the leftmost one.
//  Replace the pair with their sum.
// Return the minimum number of operations needed to make the array non-decreasing.
// An array is said to be non-decreasing if each element is greater than or equal to its previous element (if it exists).

pub struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        fn replace_first_occurence_of_pair(pair: (i32, i32), nums_array: &[i32]) -> Vec<i32> {
            let mut result: Vec<i32> = Vec::new();

            for i in 0..(nums_array.len() - 1) {
                let element_1: i32 = nums_array[i];
                let element_2: i32 = nums_array[i + 1];

                if (element_1, element_2) == pair {
                    result.extend_from_slice(&nums_array[0..i]);
                    result.push(element_1 + element_2);
                    result.extend_from_slice(&nums_array[i + 2..]);
                    return result;
                }
            }

            nums_array.to_owned()
        }
        let mut counter: i32 = 0;

        while !nums.is_sorted() {
            let mut minimum_sum_pair: (i32, i32) = (i32::MAX, i32::MAX);
            let mut minimum_sum_pair_sum: i32 = i32::MAX;

            for i in 0..(nums.len() - 1) {
                let element_1: i32 = nums[i];
                let element_2: i32 = nums[i + 1];

                if element_1 + element_2 < minimum_sum_pair_sum {
                    minimum_sum_pair_sum = element_1 + element_2;
                    minimum_sum_pair = (element_1, element_2);
                }
            }
            nums = replace_first_occurence_of_pair(minimum_sum_pair, &nums);
            counter += 1;
        }
        counter
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_pair_removal() {
        assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
        assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0)
    }
}
