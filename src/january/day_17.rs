// https://leetcode.com/problems/find-the-largest-area-of-square-inside-two-rectangles/description/?envType=daily-question&envId=2026-01-17
// 3047. Find the Largest Area of Square Inside Two Rectangles
// Medium
// There exist n rectangles in a 2D plane with edges parallel to the x and y axis.
// You are given two 2D integer arrays bottomLeft and topRight
// where bottomLeft[i] = [a_i, b_i] and topRight[i] = [c_i, d_i]
// represent the bottom-left and top-right coordinates of the ith rectangle, respectively.
// You need to find the maximum area of a square that can fit inside
// the intersecting region of at least two rectangles. Return 0 if such a square does not exist.

pub struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        fn square_intersection_area(
            rect_1: ((i32, i32), (i32, i32)),
            rect_2: ((i32, i32), (i32, i32)),
        ) -> Option<i64> {
            let rect_1_bottom_left: (i32, i32) = rect_1.0;
            let rect_1_top_right: (i32, i32) = rect_1.1;

            let rect_2_bottom_left: (i32, i32) = rect_2.0;
            let rect_2_top_right: (i32, i32) = rect_2.1;

            let intersection_bottom_left_x: i32 = rect_1_bottom_left.0.max(rect_2_bottom_left.0);
            let intersection_bottom_left_y: i32 = rect_1_bottom_left.1.max(rect_2_bottom_left.1);
            let intersection_top_right_x: i32 = rect_1_top_right.0.min(rect_2_top_right.0);
            let intersection_top_right_y = rect_1_top_right.1.min(rect_2_top_right.1);

            if intersection_bottom_left_x >= intersection_top_right_x
                || intersection_bottom_left_y >= intersection_top_right_y
            {
                return None;
            }

            // Intersection
            let intersection_width: i32 = intersection_top_right_x - intersection_bottom_left_x;
            let intersection_height: i32 = intersection_top_right_y - intersection_bottom_left_y;

            let square_length: i64 = i32::min(intersection_height, intersection_width) as i64;

            Some(square_length * square_length)
        }

        let num_rectangles: usize = bottom_left.len();
        let mut max_square_intersection_width: i64 = 0;

        for i in 0..num_rectangles {
            for j in i + 1..num_rectangles {
                let rect_1: ((i32, i32), (i32, i32)) = (
                    (bottom_left[i][0], bottom_left[i][1]),
                    (top_right[i][0], top_right[i][1]),
                );
                let rect_2: ((i32, i32), (i32, i32)) = (
                    (bottom_left[j][0], bottom_left[j][1]),
                    (top_right[j][0], top_right[j][1]),
                );
                if let Some(square_intersection_area) = square_intersection_area(rect_1, rect_2) {
                    max_square_intersection_width =
                        i64::max(max_square_intersection_width, square_intersection_area);
                }
            }
        }
        max_square_intersection_width
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_square_area() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![3, 1]],
                vec![vec![3, 3], vec![4, 4], vec![6, 6]]
            ),
            1
        );

        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![1, 3], vec![1, 5]],
                vec![vec![5, 5], vec![5, 7], vec![5, 9]]
            ),
            4
        );

        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![1, 2]],
                vec![vec![3, 3], vec![4, 4], vec![3, 4]]
            ),
            1
        );

        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![3, 3], vec![3, 1]],
                vec![vec![2, 2], vec![4, 4], vec![4, 2]]
            ),
            0
        )
    }
}
