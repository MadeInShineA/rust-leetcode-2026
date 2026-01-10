// https://leetcode.com/problems/plus-one/?envType=daily-question&envId=2026-01-01
// 66. Plus One
// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer.
// The digits are ordered from most significant to least significant in left-to-right order.
// The large integer does not contain any leading 0's.
// Increment the large integer by one and return the resulting array of digits.

pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }

        let mut result: Vec<i32> = vec![1];
        result.extend(digits);
        result
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
        assert_eq!(
            Solution::plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1]
        )
    }
}
