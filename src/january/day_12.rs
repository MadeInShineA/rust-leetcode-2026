// https://leetcode.com/problems/minimum-time-visiting-all-points/description/?envType=daily-question&envId=2026-01-12
// 1266. Minimum Time Visiting All Points
// Easy
// On a 2D plane, there are n points with integer coordinates points[i] = [xi, yi].
// Return the minimum time in seconds to visit all the points in the order given by points.
// You can move according to these rules:
//  In 1 second, you can either:
//      move vertically by one unit,
//      move horizontally by one unit, or
//      move diagonally sqrt(2) units (in other words, move one unit vertically then one unit horizontally in 1 second).
//  You have to visit the points in the same order as they appear in the array.
//  You are allowed to pass through points that appear later in the order, but these do not count as visits.

pub struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        fn min_time_to_visite_point(origin: Vec<i32>, destination: Vec<i32>) -> i32 {
            i32::max(
                i32::abs(origin[0] - destination[0]),
                i32::abs(origin[1] - destination[1]),
            )
        }
        let mut sum: i32 = 0;

        for i in 0..(points.len() - 1) {
            sum += min_time_to_visite_point(points[i].clone(), points[i + 1].clone());
        }
        sum
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_min_time_to_visit_all_points() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
    }
}
