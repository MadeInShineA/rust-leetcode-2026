// https://leetcode.com/problems/separate-squares-i/description/?envType=daily-question&envId=2026-01-13
// 3453. Separate Squares I
// Medium
// You are given a 2D integer array squares. Each squares[i] = [xi, yi, li]
// represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
// Find the minimum y-coordinate value of a horizontal line such that
// the total area of the squares above the line equals the total area of the squares below the line.
// Answers within 10-5 of the actual answer will be accepted.
// Note: Squares may overlap. Overlapping areas should be counted multiple times.
use core::f64;
pub struct Solution;

pub const EPSILON: f64 = 1e-5;

impl Solution {
    pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() <= epsilon
    }

    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        /*
        // Calculate the area above and under the y split coordinate
        fn calculate_area(split_coordinate: f64, squares: &Vec<Vec<i32>>) -> (f64, f64) {
            let mut total_under_area: f64 = 0.0;
            let mut total_above_area: f64 = 0.0;

            for square in squares {
                let bottom_left_y: f64 = square[1] as f64;
                let side_length: f64 = square[2] as f64;
                let top_left_y: f64 = bottom_left_y + side_length;

                let under_split_coordinate_height: f64 = {
                    if split_coordinate <= bottom_left_y {
                        0.0
                    } else if bottom_left_y < split_coordinate && split_coordinate < top_left_y {
                        split_coordinate - bottom_left_y

                    // split_coordinate >= top_left_y
                    } else {
                        side_length
                    }
                };

                let under_area: f64 = under_split_coordinate_height * side_length;
                let above_area: f64 = (side_length * side_length) - under_area;
                total_under_area += under_area;
                total_above_area += above_area;
            }

            (total_under_area, total_above_area)
        }


        // Dichotomic search for point where under_area = above_area
        let (mut lower_bound, mut higher_bound): (f64, f64) = squares.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY),
            |(lower_bound, higher_bound), square| {
                let bottom_left_y: f64 = square[1] as f64;
                let side_length: f64 = square[2] as f64;
                let top_left_y: f64 = bottom_left_y + side_length;
                (
                    f64::min(lower_bound, bottom_left_y),
                    f64::max(higher_bound, top_left_y),
                )
            },
        );

        let mut split_coordinate: f64 = (lower_bound + higher_bound) / 2.0;
        let (mut total_under_area, mut total_above_area): (f64, f64) = (0.0, f64::INFINITY);

        while !Solution::approx_equal(total_under_area, total_above_area, EPSILON) {
            (total_under_area, total_above_area) = calculate_area(split_coordinate, &squares);

            if Solution::approx_equal(total_under_area, total_above_area, EPSILON) {
                break;
            } else if total_under_area < total_above_area {
                lower_bound = split_coordinate
            } else if total_under_area > total_above_area {
                higher_bound = split_coordinate
            }
            split_coordinate = (higher_bound + lower_bound) / 2.0
        }

        // Refine the solution to the minimum y-coordinate where areas are balanced,
        // which may be a square boundary or an interior point.
        let mut candidates: Vec<f64> = squares
            .iter()
            .flat_map(|square| {
                let bottom = square[1] as f64;
                let top = bottom + square[2] as f64;
                vec![bottom, top]
            })
            .filter(|&y| y <= split_coordinate)
            .collect();

        candidates.push(split_coordinate);

        candidates.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for y_cand in candidates {
            let (under, above) = calculate_area(y_cand, &squares);
            if Solution::approx_equal(under, above, EPSILON) {
                return y_cand;
            }
        }
        unreachable!()
        */

        let total_area: f64 = squares
            .iter()
            .map(|square| {
                let side_length: f64 = square[2] as f64;
                side_length * side_length
            })
            .sum();

        let target_area: f64 = total_area / 2.0;

        if total_area == 0.0 {
            return 0.0;
        }

        // (y coordinate, under height change)
        let mut points_of_interest: Vec<(f64, f64)> = Vec::new();

        for square in &squares {
            let bottom_y: f64 = square[1] as f64;
            let side_length: f64 = square[2] as f64;
            let top_y: f64 = bottom_y + side_length;

            points_of_interest.push((bottom_y, side_length));
            points_of_interest.push((top_y, -side_length));
        }

        points_of_interest.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut current_height: f64 = points_of_interest[0].0;
        let mut active_side_length: f64 = 0.0; // total side lengths of active squares
        let mut current_area: f64 = 0.0;

        for (i, point_of_interest) in points_of_interest.iter().enumerate() {
            let point_height: f64 = point_of_interest.0;
            let side_length_change: f64 = point_of_interest.1; // renamed for clarity

            // Process the interval [current_height, point_height] if it has positive height
            if i > 0 && point_height > current_height {
                let height_difference: f64 = point_height - current_height;
                let area_in_interval: f64 = active_side_length * height_difference;

                let next_area = current_area + area_in_interval;

                // Check if target area is within this interval
                if current_area <= target_area && next_area >= target_area {
                    // Solve: current_area + active_side_length * delta = target_area
                    let delta = (target_area - current_area) / active_side_length;
                    return current_height + delta;
                }

                // Accumulate area for next interval
                current_area = next_area;
            }

            // Apply the event: update active side length
            active_side_length += side_length_change;
            current_height = point_height;

            // Check if we hit the target exactly at this boundary point
            if (current_area - target_area).abs() <= 1e-9 {
                return current_height;
            }
        }

        unreachable!()

        // We know that the area is changing
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_separate_squares() {
        assert!(Solution::approx_equal(
            Solution::separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]),
            1.00000,
            EPSILON,
        ));

        assert!(Solution::approx_equal(
            Solution::separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]),
            1.16667,
            EPSILON
        ));
    }
}
