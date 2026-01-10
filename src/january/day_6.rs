// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/description/?envType=daily-question&envId=2026-01-06
// 1161. Maximum Level Sum of a Binary Tree
// Medium
// Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.
// Return the smallest level x such that the sum of all the values of nodes at level x is maximal.

use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut max_depth_sum: i32 = i32::MIN;
        let mut max_sum_depth: i32 = 0;
        let mut current_depth: i32 = 1;

        if let Some(root_node) = root {
            queue.push_back(root_node);
        };

        while !queue.is_empty() {
            let depth_size: usize = queue.len();
            let mut depth_sum: i32 = 0;

            for _ in 0..depth_size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();
                depth_sum += node_ref.val;

                if let Some(left_child) = &node_ref.left {
                    queue.push_back(left_child.to_owned());
                }

                if let Some(right_child) = &node_ref.right {
                    queue.push_back(right_child.to_owned());
                }
            }

            if depth_sum > max_depth_sum {
                max_depth_sum = depth_sum;
                max_sum_depth = current_depth;
            }
            current_depth += 1;
        }

        max_sum_depth
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn rc_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn test_max_level_sum() {
        // [1,7,0,7,-8,null,null] => 2
        let root1 = rc_node(1);
        {
            let mut root_ref = root1.as_ref().unwrap().borrow_mut();
            root_ref.left = rc_node(7);
            root_ref.right = rc_node(0);
        }
        {
            let left_rc = {
                let root_borrow = root1.as_ref().unwrap().borrow();
                root_borrow.left.as_ref().unwrap().clone()
            };
            let mut left_ref = left_rc.borrow_mut();
            left_ref.left = rc_node(7);
            left_ref.right = rc_node(-8);
        }
        assert_eq!(Solution::max_level_sum(root1), 2);

        // [989,null,10250,98693,-89388,null,null,null,-32127] => 2
        let root2 = rc_node(989);
        {
            let mut root_ref = root2.as_ref().unwrap().borrow_mut();
            root_ref.right = rc_node(10250);
        }
        {
            let right_rc = {
                let root_borrow = root2.as_ref().unwrap().borrow();
                root_borrow.right.as_ref().unwrap().clone()
            };
            let mut right_ref = right_rc.borrow_mut();
            right_ref.left = rc_node(98693);
            right_ref.right = rc_node(-89388);
        }
        {
            // Get Rc handle to 98693 (left child of 10250)
            let left_grandchild_rc = {
                let root_borrow = root2.as_ref().unwrap().borrow();
                let right_rc = root_borrow.right.as_ref().unwrap();
                let right_borrow = right_rc.borrow();
                right_borrow.left.as_ref().unwrap().clone()
            };
            let mut left_grandchild_ref = left_grandchild_rc.borrow_mut();
            left_grandchild_ref.left = rc_node(-32127);
        }
        assert_eq!(Solution::max_level_sum(root2), 2);
    }
}
