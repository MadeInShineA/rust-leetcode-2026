// https://leetcode.com/problems/construct-the-minimum-bitwise-array-i/description/?envType=daily-question&envId=2026-01-20
// 3314. Construct the Minimum Bitwise Array I
// Easy
// You are given an array nums consisting of n prime integers.
// You need to construct an array ans of length n, such that,
// for each index i, the bitwise OR of ans[i] and ans[i] + 1 is equal to nums[i]
// i.e. ans[i] OR (ans[i] + 1) == nums[i].
// Additionally, you must minimize each value of ans[i] in the resulting array.
// If it is not possible to find such a value for ans[i] that satisfies the condition, then set ans[i] = -1.

pub struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();

        for element in nums {
            let mut found_answer: bool = false;
            for answer_candidate in 0..element {
                if answer_candidate | (answer_candidate + 1) == element {
                    answer.push(answer_candidate);
                    found_answer = true;
                    break;
                }
            }
            if !found_answer {
                answer.push(-1)
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
