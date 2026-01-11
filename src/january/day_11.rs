// https://leetcode.com/problems/maximal-rectangle/description/?envType=daily-question&envId=2026-01-11
// 85. Maximal Rectangle
// Hard
// Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing
// only 1's and return its area.

pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let mut int_matrix: Vec<Vec<i32>> = matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|element| element.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();

        let row: usize = int_matrix.len();
        let col: usize = int_matrix[0].len();
        let mut max_element: i32 = 0;
        let mut consecutive_down: Vec<i32> = vec![0; col];

        for i in 0..row {
            let mut consecutive_left: i32 = 0;

            for j in 0..col {
                let current_element: i32 = int_matrix[i][j];

                if current_element != 0 {
                    // Update consecutive_down (height of consecutive 1's ending at (i,j))
                    consecutive_down[j] += 1;
                    consecutive_left += 1;

                    // Now find the maximum rectangle ending at (i,j)
                    let mut min_height = consecutive_down[j];
                    let mut max_area_here = min_height; // width = 1

                    // Check all possible widths from 2 to consecutive_left
                    let max_width = consecutive_left.min(j as i32 + 1);
                    for width in 2..=max_width {
                        // Get height at position j - width + 1
                        let height_at_start = consecutive_down[j - (width - 1) as usize];
                        min_height = min_height.min(height_at_start);
                        let area = min_height * width;
                        max_area_here = max_area_here.max(area);
                    }

                    int_matrix[i][j] = max_area_here;
                    max_element = max_element.max(max_area_here);
                } else {
                    consecutive_left = 0;
                    consecutive_down[j] = 0;
                }
            }
        }

        max_element
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_maximal_rectangle() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );

        assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0', '1']]), 1);
        assert_eq!(
            Solution::maximal_rectangle(vec![vec!['1', '1'], vec!['1', '1']]),
            4
        );
    }
}
