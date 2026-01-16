// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/description/?envType=daily-question&envId=2026-01-16
// 2975. Maximum Square Area by Removing Fences From a Field
// Medium
// There is a large (m - 1) x (n - 1) rectangular field with corners at (1, 1) and (m, n)
// containing some horizontal and vertical fences given in arrays hFences and vFences respectively.
// horizontal fences are from the coordinates (hFences[i], 1) to (hFences[i], n)
// and vertical fences are from the coordinates (1, vFences[i]) to (m, vFences[i]).
// Return the maximum area of a square field that can be formed by removing some fences
// (possibly none) or -1 if it is impossible to make a square field.
// Since the answer may be large, return it modulo 109 + 7.
// Note: The field is surrounded by two horizontal fences from the coordinates (1, 1) to (1, n)
// and (m, 1) to (m, n) and two vertical fences from the coordinates
// (1, 1) to (m, 1) and (1, n) to (m, n). These fences cannot be removed.

use std::collections::HashSet;

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn maximize_square_area(
        m: i32,
        n: i32,
        mut h_fences: Vec<i32>,
        mut v_fences: Vec<i32>,
    ) -> i32 {
        h_fences.push(1);
        h_fences.push(m);
        let h_fences_length = h_fences.len();

        v_fences.push(1);
        v_fences.push(n);
        let v_fences_length = v_fences.len();

        let h_fences_diffs: HashSet<i32> = {
            let mut diff_set: HashSet<i32> = HashSet::new();

            for i in 0..h_fences_length {
                for j in 0..h_fences_length {
                    if i == j {
                    } else {
                        let elem_1: i32 = h_fences[i];
                        let elem_2: i32 = h_fences[j];
                        let diff: i32 = elem_1 - elem_2;

                        if diff > 0 {
                            diff_set.insert(diff);
                        }
                    }
                }
            }

            diff_set
        };

        let v_fences_diffs: HashSet<i32> = {
            let mut diff_set: HashSet<i32> = HashSet::new();

            for i in 0..v_fences_length {
                for j in 0..v_fences_length {
                    if i == j {
                    } else {
                        let elem_1: i32 = v_fences[i];
                        let elem_2: i32 = v_fences[j];
                        let diff: i32 = elem_1 - elem_2;

                        if diff > 0 {
                            diff_set.insert(diff);
                        }
                    }
                }
            }

            diff_set
        };

        let mut max_common_element: i32 = -1;

        for &h_fance_dif in &h_fences_diffs {
            if v_fences_diffs.contains(&h_fance_dif) {
                max_common_element = i32::max(max_common_element, h_fance_dif)
            }
        }

        if max_common_element == -1 {
            -1
        } else {
            ((max_common_element as i64 * max_common_element as i64) % MOD) as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximize_square_area() {
        assert_eq!(Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
        assert_eq!(Solution::maximize_square_area(6, 7, vec![2], vec![4]), -1);
    }
}
