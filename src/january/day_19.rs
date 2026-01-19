// https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/description/?envType=daily-question&envId=2026-01-19
// 1292. Maximum Side Length of a Square with Sum Less than or Equal to Threshold
// Medium
// Given a m x n matrix mat and an integer threshold,
// return the maximum side-length of a square with a sum less than or equal to threshold
// or return 0 if there is no such square.

pub struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        /*
                fn is_square_sum_less_equal_threshold(square: Vec<Vec<i32>>, threshold: i32) -> bool {
                    square.iter().flatten().sum::<i32>() <= threshold
                }

                let rows: usize = mat.len();
                let cols: usize = mat[0].len();

                let max_k: usize = usize::min(rows, cols);

                for k in (1..=max_k).rev() {
                    for i in 0..=(rows - k) {
                        for j in 0..=(cols - k) {
                            let square: Vec<Vec<i32>> = (0..k)
                                .map(|di| (0..k).map(|dj| mat[i + di][j + dj]).collect())
                                .collect();
                            if is_square_sum_less_equal_threshold(square, threshold) {
                                return k as i32;
                            }
                        }
                    }
                }
                0
        */

        let rows: usize = mat.len();
        let cols: usize = mat[0].len();
        let mut prefix_sum: Vec<Vec<i32>> = vec![vec![0; cols + 1]; rows + 1];

        for i in 0..rows {
            for j in 0..cols {
                prefix_sum[i + 1][j + 1] =
                    mat[i][j] + prefix_sum[i + 1][j] + prefix_sum[i][j + 1] - prefix_sum[i][j];
            }
        }

        let max_k = usize::min(rows, cols);
        let mut low: usize = 0;
        let mut high: usize = max_k;

        let compute_subsquare_sum = |i: usize, j: usize, k: usize| -> i32 {
            prefix_sum[i + k][j + k] - prefix_sum[i + k][j] - prefix_sum[i][j + k]
                + prefix_sum[i][j]
        };

        while low <= high {
            let mid: usize = (low + high) / 2;
            let mut possible: bool = false;
            'outer: for i in 0..=(rows - mid) {
                for j in 0..=(cols - mid) {
                    if compute_subsquare_sum(i, j, mid) <= threshold {
                        possible = true;
                        break 'outer;
                    }
                }
            }
            if possible {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        high as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_slide_length() {
        assert_eq!(
            Solution::max_side_length(
                vec![
                    vec![1, 1, 3, 2, 4, 3, 2],
                    vec![1, 1, 3, 2, 4, 3, 2],
                    vec![1, 1, 3, 2, 4, 3, 2]
                ],
                4
            ),
            2
        );
        assert_eq!(
            Solution::max_side_length(
                vec![
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2]
                ],
                1
            ),
            0
        )
    }
}
