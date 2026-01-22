// https://leetcode.com/problems/construct-the-minimum-bitwise-array-ii/description/?envType=daily-question&envId=2026-01-21
// 3314. Construct the Minimum Bitwise Array II
// Medium
// You are given an array nums consisting of n prime integers.
// You need to construct an array ans of length n, such that,
// for each index i, the bitwise OR of ans[i] and ans[i] + 1 is equal to nums[i]
// i.e. ans[i] OR (ans[i] + 1) == nums[i].
// Additionally, you must minimize each value of ans[i] in the resulting array.
// If it is not possible to find such a value for ans[i] that satisfies the condition, then set ans[i] = -1.

pub struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        // Elements are prime so there binary representation ends in 1 (Except for 2 but 2 answer
        // is -1)
        // xxxxxx OR (xxxxxx + 00001) = yyyyyy1 (element)
        // If element = 00000101
        // xxxxxxxx OR (xxxxxxxx + 00000001) = 00000101
        // => answer = 000000xxx
        //
        // 3 => 1
        // 00000011 => 00000001
        // 5 => 4
        // 00000101 => 00000100
        // 7 => 3
        // 00000111  => 00000011
        // 11 => 9
        // 00001011 => 00001001
        // 13 => 12
        // 00001101 => 00001100
        // 31 => 15
        // 00011111 => 00001111
        // From Right to left, set the first 1 on the right of a 0 to 1 if element is 2 => -1
        let mut answer: Vec<i32> = Vec::new();

        for element in nums {
            if element == 2 {
                answer.push(-1);
            } else {
                let binary_string: String = format!("0{:b}", element);
                let mut has_flipped: bool = false;
                let mut result_binary_string: String = String::new();

                for bit in binary_string.chars().rev() {
                    if bit == '0' && !has_flipped {
                        result_binary_string.pop();
                        result_binary_string.push('0');
                        has_flipped = true;
                    }
                    result_binary_string.push(bit);
                }

                let result_binary_string: String = result_binary_string.chars().rev().collect();
                let result_i32: i32 = i32::from_str_radix(&result_binary_string, 2).unwrap();
                answer.push(result_i32);
            }
        }
        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_bitwise_array() {
        assert_eq!(
            Solution::min_bitwise_array(vec![2, 3, 5, 7]),
            vec![-1, 1, 4, 3]
        );

        assert_eq!(
            Solution::min_bitwise_array(vec![11, 13, 31]),
            vec![9, 12, 15]
        )
    }
}

