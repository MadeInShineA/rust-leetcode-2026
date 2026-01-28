// https://leetcode.com/problems/minimum-cost-path-with-teleportations/description/?envType=daily-question&envId=2026-01-28
// 3651. Minimum Cost Path with Teleportations
// Hard
// You are given a m x n 2D integer array grid and an integer k.
// You start at the top-left cell (0, 0) and your goal is to reach the bottom‐right cell (m - 1, n - 1).
// There are two types of moves available:
//  Normal move: You can move right or down from your current cell (i, j),
//      you can move to (i, j + 1) (right) or (i + 1, j) (down). The cost is the value of the destination cell.
//  Teleportation: You can teleport from any cell (i, j), to any cell (x, y)
//      uch that grid[x][y] <= grid[i][j]; the cost of this move is 0. You may teleport at most k times.
// Return the minimum total cost to reach cell (m - 1, n - 1) from (0, 0).

// use std::cmp::Reverse;
// use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        // let n = grid.len() as i32;
        // let m = grid[0].len() as i32;
        // let max_k = k as usize;
        // // dist[(i, j, t)] = minimum cost to reach (i, j) using exactly t teleports
        // let mut distances: HashMap<(i32, i32, usize), i32> = HashMap::new();
        // let mut heap = BinaryHeap::new();
        //
        // // Start at (0, 0) with 0 cost and 0 teleports used
        // distances.insert((0, 0, 0), 0);
        // heap.push(Reverse((0, 0, 0, 0)));
        //
        // while let Some(Reverse((cost, i, j, t))) = heap.pop() {
        //     // Skip if we already found a better way to this state
        //     if distances.get(&(i, j, t)).copied().unwrap_or(i32::MAX) < cost {
        //         continue;
        //     }
        //
        //     // Check if reached destination
        //     if i == n - 1 && j == m - 1 {
        //         return cost;
        //     }
        //
        //     // Normal moves: down and right
        //     for &(di, dj) in &[(1, 0), (0, 1)] {
        //         let ni = i + di;
        //         let nj = j + dj;
        //         if ni < n && nj < m {
        //             let next_cost = cost + grid[ni as usize][nj as usize];
        //             if next_cost < *distances.get(&(ni, nj, t)).unwrap_or(&i32::MAX) {
        //                 distances.insert((ni, nj, t), next_cost);
        //                 heap.push(Reverse((next_cost, ni, nj, t)));
        //             }
        //         }
        //     }
        //
        //     // Teleportation: use one teleport if we have any left
        //     if t < max_k {
        //         let current_val = grid[i as usize][j as usize];
        //         for x in 0..n {
        //             for y in 0..m {
        //                 if x == i && y == j {
        //                     continue;
        //                 }
        //                 if grid[x as usize][y as usize] <= current_val {
        //                     // Teleport cost is 0, so total cost remains `cost`
        //                     if cost < *distances.get(&(x, y, t + 1)).unwrap_or(&i32::MAX) {
        //                         distances.insert((x, y, t + 1), cost);
        //                         heap.push(Reverse((cost, x, y, t + 1)));
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
        // unreachable!()
        let mut g = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        g[0][0] = 0;
        let downright_propagation = |g: &mut [Vec<i32>]| {
            let mut grid_max = 0;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    grid_max = grid_max.max(grid[i][j]);
                    if i > 0 && j > 0 {
                        g[i][j] = g[i][j].min(g[i - 1][j].min(g[i][j - 1]) + grid[i][j]);
                    } else if i > 0 {
                        g[i][j] = g[i][j].min(g[i - 1][j] + grid[i][j]);
                    } else if j > 0 {
                        g[i][j] = g[i][j].min(g[i][j - 1] + grid[i][j]);
                    }
                }
            }
            grid_max
        };
        let grid_max = downright_propagation(&mut g) as usize;
        for _ in 0..k {
            let mut min_steps_for_level = vec![i32::MAX; grid_max + 1];
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    min_steps_for_level[grid[i][j] as usize] =
                        min_steps_for_level[grid[i][j] as usize].min(g[i][j]);
                }
            }
            for i in (0..grid_max).rev() {
                min_steps_for_level[i] = min_steps_for_level[i].min(min_steps_for_level[i + 1]);
            }
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    g[i][j] = g[i][j].min(min_steps_for_level[grid[i][j] as usize]);
                }
            }
            downright_propagation(&mut g);
        }
        *g.last().unwrap().last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_cost() {
        assert_eq!(
            Solution::min_cost(vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]], 2),
            7
        );
        assert_eq!(
            Solution::min_cost(vec![vec![1, 2], vec![2, 3], vec![3, 4]], 1),
            9
        );
    }
}
