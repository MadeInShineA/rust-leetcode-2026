// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/description/?envType=daily-question&envId=2026-01-09
// 865. Smallest Subtree with all the Deepest Nodes
// Given the root of a binary tree, the depth of each node is the shortest distance to the root.
// Return the smallest subtree such that it contains all the deepest nodes in the original tree.
// A node is called the deepest if it has the largest depth possible among any node in the entire tree.
// The subtree of a node is a tree consisting of that node, plus the set of all descendants of that node.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Use a post-order traversal (DFS) instead of BFS.
        // For each node, recursively compute:
        //   - the maximum depth of its left and right subtrees,
        //   - and the candidate answer (smallest subtree containing all deepest nodes) in each.
        // At the current node:
        //   - If both subtrees have the same max depth, then this node is the LCA of all deepest nodes in its subtree → it becomes the new candidate.
        //   - If one subtree is deeper, then all deepest nodes lie in that side → propagate its candidate upward.
        // The final candidate returned at the root is the answer.
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node_element) = node {
                let (left_max_depth, left_candidate) = dfs(node_element.borrow().left.clone());
                let (right_max_depth, right_candidate) = dfs(node_element.borrow().right.clone());

                if left_max_depth == right_max_depth {
                    (left_max_depth + 1, Some(node_element))
                } else if left_max_depth > right_max_depth {
                    (left_max_depth + 1, left_candidate)
                } else {
                    (right_max_depth + 1, right_candidate)
                }
            } else {
                (0, None)
            }
        }
        let (_, result) = dfs(root);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn tree_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_subtree_with_deepest() {
        // Test case 1: root = [3,5,1,6,2,0,8,null,null,7,4]
        // Expected output: [2,7,4]
        let node7 = tree_node(7, None, None);
        let node4 = tree_node(4, None, None);
        let node2 = tree_node(2, node7, node4);
        let node6 = tree_node(6, None, None);
        let node5 = tree_node(5, node6, node2);
        let node0 = tree_node(0, None, None);
        let node8 = tree_node(8, None, None);
        let node1 = tree_node(1, node0, node8);
        let root1 = tree_node(3, node5, node1);

        let result1 = Solution::subtree_with_all_deepest(root1);
        assert_eq!(result1.as_ref().unwrap().borrow().val, 2);
        assert_eq!(
            result1
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            7
        );
        assert_eq!(
            result1
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            4
        );

        // Test case 2: root = [1]
        // Expected output: [1]
        let root2 = tree_node(1, None, None);
        let result2 = Solution::subtree_with_all_deepest(root2);
        assert_eq!(result2.as_ref().unwrap().borrow().val, 1);

        // Test case 3: root = [0,1,3,null,2]
        // Expected output: [2]
        let node2 = tree_node(2, None, None);
        let node1 = tree_node(1, None, node2);
        let node3 = tree_node(3, None, None);
        let root3 = tree_node(0, node1, node3);
        let result3 = Solution::subtree_with_all_deepest(root3);
        assert_eq!(result3.as_ref().unwrap().borrow().val, 2);
    }
}
