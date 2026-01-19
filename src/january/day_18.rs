// https://leetcode.com/problems/largest-magic-square/description/?envType=daily-question&envId=2026-01-18
// 1895. Largest Magic Square
// Medium
// A k x k magic square is a k x k grid filled with integers such that
// every row sum, every column sum, and both diagonal sums are all equal.
// The integers in the magic square do not have to be distinct. Every 1 x 1 grid is trivially a magic square.
// Given an m x n integer grid, return the size (i.e., the side length k) of the largest magic square that can be found within this grid.

pub struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        fn is_magic_square(square: Vec<Vec<i32>>) -> bool {
            let k: usize = square.len();
            let target_sum: i32 = square[0].iter().sum();
            let mut lr_diagonal_sum: i32 = 0;
            let mut rl_diagonal_sum: i32 = 0;

            for (i, row) in square.iter().enumerate() {
                if row.iter().sum::<i32>() != target_sum {
                    return false;
                }
                for (j, value) in row.iter().enumerate() {
                    let column_sum: i32 = (0..k).map(|i| square[i][j]).sum();

                    if column_sum != target_sum {
                        return false;
                    }

                    if i == j {
                        lr_diagonal_sum += value;
                    }
                    if i + j == k - 1 {
                        rl_diagonal_sum += value;
                    }
                }
            }

            if lr_diagonal_sum != target_sum {
                return false;
            }

            if rl_diagonal_sum != target_sum {
                return false;
            }
            true
        }

        let rows: usize = grid.len();
        let cols: usize = grid[0].len();
        let max_k: usize = usize::min(rows, cols);

        for k in (1..=max_k).rev() {
            for i in 0..=(rows - k) {
                for j in 0..=(cols - k) {
                    let square: Vec<Vec<i32>> = (0..k)
                        .map(|di| (0..k).map(|dj| grid[i + di][j + dj]).collect())
                        .collect();
                    if is_magic_square(square) {
                        return k as i32;
                    }
                }
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_magic_square() {
        assert_eq!(
            Solution::largest_magic_square(vec![
                vec![7, 1, 4, 5, 6],
                vec![2, 5, 1, 6, 4],
                vec![1, 5, 4, 3, 2],
                vec![1, 2, 7, 3, 4]
            ]),
            3
        );

        assert_eq!(
            Solution::largest_magic_square(vec![
                vec![5, 1, 3, 1],
                vec![9, 3, 3, 1],
                vec![1, 3, 3, 8]
            ]),
            2
        );

        assert_eq!(
            Solution::largest_magic_square(vec![
                vec![8, 1, 6],
                vec![3, 5, 7],
                vec![4, 9, 2],
                vec![7, 10, 9]
            ]),
            3
        )
    }
}
