// https://leetcode.com/problems/minimum-cost-to-convert-string-ii/description/?envType=daily-question&envId=2026-01-30
// 2977. Minimum Cost to Convert String II
// Hard
// You are given two 0-indexed strings source and target, both of length n and consisting of lowercase English characters.
// You are also given two 0-indexed string arrays original and changed
// and an integer array cost, where cost[i] represents the cost of converting the string original[i] to the string changed[i].
// You start with the string source. In one operation, you can pick a substring x from the string, and change it to y at a cost of z
// if there exists any index j such that cost[j] == z, original[j] == x, and changed[j] == y
// You are allowed to do any number of operations, but any pair of operations must satisfy either of these two conditions:
//  The substrings picked in the operations are source[a..b] and source[c..d] with either b < c or d < a.
//      In other words, the indices picked in both operations are disjoint.
//  The substrings picked in the operations are source[a..b] and source[c..d] with a == c and b == d.
//      In other words, the indices picked in both operations are identical.
// Return the minimum cost to convert the string source to the string target using any number of operations.
// If it is impossible to convert source to target, return -1.
// Note that there may exist indices i, j such that original[j] == original[i] and changed[j] == changed[i].

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Trie {
    count_subtree: i32,
    val: i32,
    children: [Option<Box<Trie>>; 26],
}

pub struct Solution;

impl Trie {
    fn new() -> Self {
        Trie {
            count_subtree: 0,
            val: -1,
            children: Default::default(),
        }
    }

    fn insert(&mut self, word: &str, idx: i32) {
        let mut node = self;
        for &c in word.as_bytes() {
            let child_idx = (c - b'a') as usize;
            node.count_subtree += 1;
            node.children[child_idx].get_or_insert_with(|| Box::new(Trie::new()));
            node = node.children[child_idx].as_mut().unwrap();
        }
        node.count_subtree += 1;
        node.val = idx;
    }

    fn search(&self, word: &str) -> i32 {
        let mut node = self;
        for &c in word.as_bytes() {
            let child_idx = (c - b'a') as usize;
            if let Some(child) = &node.children[child_idx] {
                node = child;
            } else {
                return -1;
            }
        }
        node.val
    }
}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        const MAX: i32 = 1_000_000_005;
        const MAXL: i64 = 1_000_000_000_000_000_000;

        let n = source.len();
        let m = original.len();

        let mut vo = Vec::new();
        let mut vc = Vec::new();
        let mut trie = Trie::new();
        let mut curr_idx = 0;

        for word in &original {
            if trie.search(word) == -1 {
                trie.insert(word, curr_idx);
                curr_idx += 1;
            }
            vo.push(trie.search(word));
        }

        for word in &changed {
            if trie.search(word) == -1 {
                trie.insert(word, curr_idx);
                curr_idx += 1;
            }
            vc.push(trie.search(word));
        }

        let mut edges = vec![Vec::new(); 2 * m];
        for i in 0..m {
            let u = vo[i] as usize;
            let v = vc[i] as usize;
            edges[u].push((v, cost[i]));
        }

        let mut ct = vec![vec![MAX; 201]; 201];
        let mut processed = vec![false; 2 * m];

        for i in 0..m {
            let u = vo[i] as usize;
            if processed[u] {
                continue;
            }
            processed[u] = true;

            let mut pq = BinaryHeap::new();
            for &(v, c) in &edges[u] {
                pq.push(Reverse((c, v)));
            }

            while let Some(Reverse((c, v))) = pq.pop() {
                if c < ct[u][v] {
                    ct[u][v] = c;
                    for &(t, cc) in &edges[v] {
                        pq.push(Reverse((c + cc, t)));
                    }
                }
            }
        }

        let mut dp = vec![MAXL; n];
        for i in (0..n).rev() {
            let mut node1 = &trie;
            let mut node2 = &trie;

            if source.as_bytes()[i] == target.as_bytes()[i] {
                dp[i] = (if i + 1 == n { 0 } else { dp[i + 1] }).min(dp[i]);
            }

            for j in i..n {
                let idx1 = (source.as_bytes()[j] - b'a') as usize;
                let idx2 = (target.as_bytes()[j] - b'a') as usize;

                if node1.children[idx1].is_none() || node2.children[idx2].is_none() {
                    break;
                }

                node1 = node1.children[idx1].as_ref().unwrap();
                node2 = node2.children[idx2].as_ref().unwrap();

                let is = node1.val;
                let it = node2.val;

                if is == -1 || it == -1 || ct[is as usize][it as usize] == MAX {
                    continue;
                }

                dp[i] = dp[i].min(
                    (if j + 1 == n { 0 } else { dp[j + 1] }) + ct[is as usize][it as usize] as i64,
                );
            }
        }

        if dp[0] == MAXL { -1 } else { dp[0] }
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
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "c".to_string(),
                    "e".to_string(),
                    "d".to_string()
                ],
                vec![
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "e".to_string(),
                    "b".to_string(),
                    "e".to_string()
                ],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );

        assert_eq!(
            Solution::minimum_cost(
                String::from("abcdefgh"),
                String::from("acdeeghh"),
                vec!["bcd".to_string(), "fgh".to_string(), "thh".to_string()],
                vec!["cde".to_string(), "thh".to_string(), "ghh".to_string()],
                vec![1, 3, 5]
            ),
            9
        );

        // Test case 3
        assert_eq!(
            Solution::minimum_cost(
                String::from("abcdefgh"),
                String::from("addddddd"),
                vec!["bcd".to_string(), "defgh".to_string()],
                vec!["ddd".to_string(), "ddddd".to_string()],
                vec![100, 1578]
            ),
            -1
        );
    }
}
