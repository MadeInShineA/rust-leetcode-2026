// https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/description/?envType=daily-question&envId=2026-01-15
// 2943. Maximize Area of Square Hole in Grid
// Medium
// You are given the two integers, n and m and two integer arrays, hBars and vBars.
// The grid has n + 2 horizontal and m + 2 vertical bars, creating 1 x 1 unit cells. The bars are indexed starting from 1.
// You can remove some of the bars in hBars from horizontal bars and some of the bars in vBars from vertical bars.
// Note that other bars are fixed and cannot be removed.
// Return an integer denoting the maximum area of a square-shaped hole in the grid, after removing some bars (possibly none).

use std::{collections::HashMap, ops::RangeInclusive};

pub struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(
        n: i32,
        m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        /*
        h_bars.sort();
        v_bars.sort();

        // At the moment all holes in the grid are 1x1 ones
        // Check the different holes width doable by removing the h_bars
        // Check the different holes height doable by removing the v_bars
        // Return max squared hole area

        // Vec of (Removed bars, width)
        let mut h_holes: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut v_holes: HashMap<Vec<i32>, i32> = HashMap::new();

        // For each h_bar in sorted h_bars, add (h_bar, 2)
        // If the left bar of h_bar has been removed
        //  Also add (... + h_bar, ... + 2) to all the holes that removed the left bar
        for h_bar in h_bars {
            h_holes.insert(vec![h_bar], 2);

            let items_to_insert: Vec<(Vec<i32>, i32)> = h_holes
                .iter()
                .filter(|(removed_bars, _)| removed_bars.contains(&(h_bar - 1)))
                .map(|(removed_bars, width)| {
                    let mut new_vec: Vec<i32> = removed_bars.clone();
                    new_vec.push(h_bar);
                    (new_vec, width + 1)
                })
                .collect();

            for (new_vec, new_width) in items_to_insert {
                h_holes.insert(new_vec, new_width);
            }
        }

        // We do the same for the v_bars
        for v_bar in v_bars {
            v_holes.insert(vec![v_bar], 2);

            let items_to_insert: Vec<(Vec<i32>, i32)> = v_holes
                .iter()
                .filter(|(removed_bars, _)| removed_bars.contains(&(v_bar - 1)))
                .map(|(removed_bars, width)| {
                    let mut new_vec: Vec<i32> = removed_bars.clone();
                    new_vec.push(v_bar);
                    (new_vec, width + 1)
                })
                .collect();

            for (new_vec, new_width) in items_to_insert {
                v_holes.insert(new_vec, new_width);
            }
        }

        dbg!(h_holes);
        dbg!(v_holes);
        */
        h_bars.sort();
        v_bars.sort();

        // Find the length of longest subsequence of h_bars
        let mut max_h_subsequence_length: i32 = 0;
        let mut current_h_subsequence_length: i32 = 0;

        for i in 1..h_bars.len() {
            let previous_h_bar: i32 = h_bars[i - 1];
            let current_h_bar: i32 = h_bars[i];

            if previous_h_bar + 1 == current_h_bar {
                current_h_subsequence_length += 1;
                max_h_subsequence_length =
                    i32::max(max_h_subsequence_length, current_h_subsequence_length);
            } else {
                current_h_subsequence_length = 0;
            }
        }

        // Do the same for v_bars
        let mut max_v_subsequence_length: i32 = 0;
        let mut current_v_subsequence_length: i32 = 0;

        for i in 1..v_bars.len() {
            let previous_v_bar: i32 = v_bars[i - 1];
            let current_v_bar: i32 = v_bars[i];

            if previous_v_bar + 1 == current_v_bar {
                current_v_subsequence_length += 1;
                max_v_subsequence_length =
                    i32::max(max_v_subsequence_length, current_v_subsequence_length);
            } else {
                current_v_subsequence_length = 0;
            }
        }
        dbg!(max_h_subsequence_length);
        dbg!(max_v_subsequence_length);

        i32::pow(
            i32::min(max_h_subsequence_length + 2, max_v_subsequence_length + 2),
            2,
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_maximize_square_hole_area() {
        assert_eq!(
            Solution::maximize_square_hole_area(2, 1, vec![2, 3], vec![2]),
            4
        );
        assert_eq!(
            Solution::maximize_square_hole_area(1, 1, vec![2], vec![2]),
            4
        );
        assert_eq!(
            Solution::maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 4]),
            4
        );
        assert_eq!(
            Solution::maximize_square_hole_area(3, 2, vec![3, 2, 4], vec![3, 2]),
            9
        );
        assert_eq!(
            Solution::maximize_square_hole_area(4, 40, vec![5, 3, 2, 4], vec![36, 41, 6, 34, 33]),
            9
        );
    }
}
