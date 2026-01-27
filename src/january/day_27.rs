// https://leetcode.com/problems/minimum-cost-path-with-edge-reversals/description/?envType=daily-question&envId=2026-01-27
// 3650. Minimum Cost Path with Edge Reversals
// Medium
// You are given a directed, weighted graph with n nodes labeled from 0 to n - 1,
// and an array edges where edges[i] = [ui, vi, wi] represents
// a directed edge from node ui to node vi with cost wi.
// Each node ui has a switch that can be used at most once
// when you arrive at ui and have not yet used its switch,
// you may activate it on one of its incoming edges
// vi → ui reverse that edge to ui → vi and immediately traverse it.
// The reversal is only valid for that single move, and using a reversed edge costs 2 * wi.
// Return the minimum total cost to travel from node 0 to node n - 1. If it is not possible, return -1.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct Solution;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let target_node: i32 = n - 1;

        // Graph as {idx: [(destination, cost)]}
        let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        // Build the graph
        for edge in edges {
            let start_node: i32 = edge[0];
            let destination_node: i32 = edge[1];
            let cost: i32 = edge[2];

            graph
                .entry(start_node)
                .or_default()
                .push((destination_node, cost));

            graph
                .entry(destination_node)
                .or_default()
                .push((start_node, cost * 2));
        }

        let mut distances: HashMap<i32, i32> = HashMap::new();
        // Set the source distance cost to 0
        distances.insert(0, 0);

        let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        // Set (0, src) as the first heap element
        heap.push(Reverse((0, 0)));

        // Dijkstra algorithm
        while let Some(Reverse((current_distance, current_node))) = heap.pop() {
            if current_distance > *distances.entry(current_node).or_insert(i32::MAX) {
                continue;
            }

            if let Some(neighbors) = graph.get(&current_node) {
                for &(neighbor, weight) in neighbors {
                    let distance: i32 = current_distance + weight;

                    let entry: &mut i32 = distances.entry(neighbor).or_insert(i32::MAX);
                    if distance < *entry {
                        *entry = distance;
                        heap.push(Reverse((distance, neighbor)));
                    }
                }
            }
        }

        let distances_to_target: i32 = *distances.get(&target_node).unwrap_or(&i32::MAX);
        match distances_to_target {
            i32::MAX => -1,
            _ => distances_to_target,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_cost() {
        assert_eq!(
            Solution::min_cost(
                4,
                vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]]
            ),
            5
        );
        assert_eq!(
            Solution::min_cost(
                4,
                vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]]
            ),
            3
        );
    }
}
