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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>, is_balanced: &mut bool) -> i32 {
        match root {
            Some(node) => {
                let left = Self::max_depth(node.borrow().left.clone(), is_balanced);
                let right = Self::max_depth(node.borrow().right.clone(), is_balanced);
                let diff = i32::abs(left - right);

                if diff > 1 {
                    *is_balanced = false;
                }

                1 + cmp::max(left, right)
            },
            _ => {
                -1
            }
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_balanced = true;
        Self::max_depth(root, &mut is_balanced);
        is_balanced
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
    let res = Solution::is_balanced(tree);
    println!("{}", res);
}

