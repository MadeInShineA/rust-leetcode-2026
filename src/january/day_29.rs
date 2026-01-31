// https://leetcode.com/problems/minimum-cost-to-convert-string-i/description/?envType=daily-question&envId=2026-01-29
// 2976. Minimum Cost to Convert String I
// Medium
// You are given two 0-indexed strings source and target, both of length n and consisting of lowercase English letters.
// You are also given two 0-indexed character arrays original and changed
// and an integer array cost, where cost[i] represents the cost of changing the character original[i] to the character changed[i].
// You start with the string source. In one operation, you can pick a character x from the string
// and change it to the character y at a cost of z
// if there exists any index j such that cost[j] == z, original[j] == x, and changed[j] == y.
// Return the minimum cost to convert the string source to the string target using any number of operations.
// If it is impossible to convert source to target, return -1.
// Note that there may exist indices i, j such that original[j] == original[i] and changed[j] == changed[i].

pub struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let l = original.len();
        let mut dp: Vec<Vec<i64>> = vec![vec![i64::MAX; 26]; 26];
        let a = 'a' as usize;
        for i in 0..l {
            let c1 = original[i] as usize - a;
            let c2 = changed[i] as usize - a;
            dp[c1][c2] = std::cmp::min(cost[i] as i64, dp[c1][c2]);
        }
        for i in 0..26 {
            dp[i][i] = 0;
        }

        for k in 0..26 {
            'outer: for i in 0..26 {
                for j in 0..26 {
                    if dp[i][k] == i64::MAX {
                        continue 'outer;
                    }
                    if dp[k][j] == i64::MAX {
                        continue;
                    }
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i][k] + dp[k][j]);
                }
            }
        }
        // println!("{:?}",dp);
        let mut cost: i64 = 0;
        let src = source.as_bytes();
        let tgt = target.as_bytes();
        let m = source.len();
        for i in 0..m {
            // println!("{}",dp[src[i] as usize -a ][tgt[i] as usize -a]);
            let val = dp[src[i] as usize - a][tgt[i] as usize - a];
            if val == i64::MAX {
                return -1;
            }
            cost += val;
        }
        cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(
                String::from("abcd"),
                String::from("acbe"),
                vec!['a', 'b', 'c', 'c', 'e', 'd'],
                vec!['b', 'c', 'b', 'e', 'b', 'e'],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );

        assert_eq!(
            Solution::minimum_cost(
                String::from("aaaa"),
                String::from("bbbb"),
                vec!['a', 'c'],
                vec!['c', 'b'],
                vec![1, 2]
            ),
            12
        );

        // Test case 3
        assert_eq!(
            Solution::minimum_cost(
                String::from("abcd"),
                String::from("abce"),
                vec!['a'],
                vec!['e'],
                vec![10000]
            ),
            -1
        );
    }
}
