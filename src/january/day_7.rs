// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/?envType=daily-question&envId=2026-01-07
// 1339. Maximum Product of Splitted Binary Tree
// Given the root of a binary tree, split the binary tree into two subtrees by removing one edge such that
// the product of the sums of the subtrees is maximized.
// Return the maximum product of the sums of the two subtrees.
// Since the answer may be too large, return it modulo 109 + 7.
// Note that you need to maximize the answer before taking the mod and not after taking it.
use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;

const MOD: i64 = 1_000_000_007;
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
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Construct a new tree where the new_node.value = node.value + sum(left.value) + sum(right.value)
        // Then itterate on the new tree by doing
        // max = max(max, node.left * root.value - left, node.right * root.value - right)

        fn sum_tree(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
            {
                let mut node: RefMut<TreeNode> = root.borrow_mut();

                let left_rc: Option<Rc<RefCell<TreeNode>>> = node.left.clone();
                let right_rc: Option<Rc<RefCell<TreeNode>>> = node.right.clone();

                let left_sum = if let Some(left) = left_rc {
                    sum_tree(left).borrow().val
                } else {
                    0
                };

                let right_sum = if let Some(right) = right_rc {
                    sum_tree(right).borrow().val
                } else {
                    0
                };

                node.val += left_sum + right_sum;
            }

            root
        }

        if let Some(root) = root {
            let new_tree = sum_tree(root);

            let mut cut_max: i64 = 0;
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            queue.push_back(new_tree.clone());

            while !queue.is_empty() {
                let depth_size: usize = queue.len();

                for _ in 0..depth_size {
                    let node = queue.pop_front().unwrap();
                    let node_ref = node.borrow();

                    let left_cut_value: i64 = if let Some(left) = &node_ref.left {
                        queue.push_back(left.to_owned());
                        (left.borrow().val * (new_tree.borrow().val - left.borrow().val)) as i64
                    } else {
                        0
                    };

                    let right_cut_value: i64 = if let Some(right) = &node_ref.right {
                        queue.push_back(right.to_owned());
                        (right.borrow().val * (new_tree.borrow().val - right.borrow().val)) as i64
                    } else {
                        0
                    };

                    cut_max = i64::max(i64::max(cut_max, left_cut_value), right_cut_value)
                }
            }

            (cut_max % MOD) as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn rc_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn test_max_product() {
        // Test case 1: [1,2,3,4,5,6] => 110
        let root1 = rc_node(1);
        {
            let mut root_ref = root1.as_ref().unwrap().borrow_mut();
            root_ref.left = rc_node(2);
            root_ref.right = rc_node(3);
        }
        {
            let left_rc = {
                let root_borrow = root1.as_ref().unwrap().borrow();
                root_borrow.left.as_ref().unwrap().clone()
            };
            let mut left_ref = left_rc.borrow_mut();
            left_ref.left = rc_node(4);
            left_ref.right = rc_node(5);
        }
        {
            let right_rc = {
                let root_borrow = root1.as_ref().unwrap().borrow();
                root_borrow.right.as_ref().unwrap().clone()
            };
            let mut right_ref = right_rc.borrow_mut();
            right_ref.left = rc_node(6);
            // right_ref.right remains None
        }
        assert_eq!(Solution::max_product(root1), 110);

        // Test case 2: [1,null,2,3,4,null,null,5,6] => 90
        let root2 = rc_node(1);
        {
            let mut root_ref = root2.as_ref().unwrap().borrow_mut();
            root_ref.right = rc_node(2);
        }
        {
            let right_rc = {
                let root_borrow = root2.as_ref().unwrap().borrow();
                root_borrow.right.as_ref().unwrap().clone()
            };
            let mut right_ref = right_rc.borrow_mut();
            right_ref.left = rc_node(3);
            right_ref.right = rc_node(4);
        }
        {
            // Get Rc to node 4 (RIGHT child of 2, not left!)
            let right_grandchild_rc = {
                let root_borrow = root2.as_ref().unwrap().borrow();
                let right_rc = root_borrow.right.as_ref().unwrap();
                let right_borrow = right_rc.borrow();
                right_borrow.right.as_ref().unwrap().clone() // ← .right, not .left
            };
            let mut right_grandchild_ref = right_grandchild_rc.borrow_mut();
            right_grandchild_ref.left = rc_node(5);
            right_grandchild_ref.right = rc_node(6);
        }
        assert_eq!(Solution::max_product(root2), 90);
    }
}
