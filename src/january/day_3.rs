// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/description/?envType=daily-question&envId=2026-01-03
// 1411. Number of Ways to Paint N × 3 Grid
// Hard
// You have a grid of size n x 3 and you want to paint each cell of the grid with exactly one of the three colors:
// Red, Yellow, or Green while making sure that no two adjacent cells have the same color
// (i.e., no two cells that share vertical or horizontal sides have the same color).
// Given n the number of rows of the grid, return the number of ways you can paint this grid.
// As the answer may grow large, the answer must be computed modulo 109 + 7.411. Number of Ways to Paint N × 3 Grid

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        // A type = ABC
        let mut a: i64 = 6;

        // B type = ABA
        let mut b: i64 = 6;

        // RYG =>
        // Type A
        // GRY + YRG
        // Type B
        // YRY + GRG

        // RYR =>
        // Type A
        // GRY + YRG
        // Type B
        // GRG + YGY + YRY

        for _ in 1..n {
            let new_a: i64 = (2 * a + 2 * b) % MOD;
            let new_b: i64 = (2 * a + 3 * b) % MOD;

            a = new_a;
            b = new_b
        }

        ((a + b) % MOD) as i32
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_num_of_ways() {
        assert_eq!(Solution::num_of_ways(1), 12);
        assert_eq!(Solution::num_of_ways(5000), 30228214);
    }
}
