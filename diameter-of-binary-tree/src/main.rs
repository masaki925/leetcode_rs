// Definition for a binary tree node.

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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution {}

use std::cmp;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let left_depth = Self::max_depth(node.borrow().left.clone(), max);
                let right_depth = Self::max_depth(node.borrow().right.clone(), max);

                *max = cmp::max(*max, 2 + left_depth + right_depth);

                return 1 + cmp::max(left_depth, right_depth)
            },
            _ => {
                return -1
            }
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;

        Self::max_depth(root, &mut max);

        max
    }
}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode { 
                val: 1,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode { 
                val: 3,
                left: None,
                right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode { 
                val: 6,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode { 
                val: 9,
                left: None,
                right: None
            })))
        })))
    })));
    let res = Solution::diameter_of_binary_tree(tree);
    println!("{}", res);
}

