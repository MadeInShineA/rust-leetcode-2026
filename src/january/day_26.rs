// https://leetcode.com/problems/minimum-absolute-difference/description/?envType=daily-question&envId=2026-01-26
// 1200. Minimum Absolute Difference
// Easy
// Given an array of distinct integers arr, find all pairs of elements
// with the minimum absolute difference of any two elements.
// Return a list of pairs in ascending order(with respect to pairs), each pair [a, b] follows
//  a, b are from arr
//  a < b
//  b - a equals to the minimum absolute difference of any two elements in arr

pub struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();

        let n: usize = arr.len();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut min_abs_diff: u32 = u32::MAX;

        for i in 0..(n - 1) {
            let element_1: i32 = arr[i];
            let element_2: i32 = arr[i + 1];

            let abs_diff: u32 = element_1.abs_diff(element_2);

            if abs_diff == min_abs_diff {
                result.push(vec![element_1, element_2]);
            } else if abs_diff < min_abs_diff {
                result = vec![vec![element_1, element_2]];
                min_abs_diff = abs_diff;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_abs_difference() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
            vec![vec![1, 2], vec![2, 3], vec![3, 4]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
            vec![vec![1, 3]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]]
        );
    }
}
