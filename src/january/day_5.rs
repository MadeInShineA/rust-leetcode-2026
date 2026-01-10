// https://leetcode.com/problems/maximum-matrix-sum/?envType=daily-question&envId=2026-01-05
// 1975. Maximum Matrix Sum
// You are given an n x n integer matrix. You can do the following operation any number of times:
//  Choose any two adjacent elements of matrix and multiply each of them by -1.
// Two elements are considered adjacent if and only if they share a border.
// Your goal is to maximize the summation of the matrix's elements.
// Return the maximum sum of the matrix's elements using the operation mentioned above.
pub struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut min: i32 = i32::MAX;
        let mut neg_count: i32 = 0;
        let mut sum: i64 = 0;

        for row in matrix {
            for element in row {
                if element < 0 {
                    neg_count += 1;
                    sum -= element as i64;
                } else {
                    sum += element as i64;
                }

                if i32::abs(element) < min {
                    min = i32::abs(element);
                }
            }
        }

        if neg_count % 2 == 0 {
            sum
        } else {
            sum - (2 * min) as i64
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_matrix_sum() {
        assert_eq!(Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
        assert_eq!(
            Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        )
    }
}
