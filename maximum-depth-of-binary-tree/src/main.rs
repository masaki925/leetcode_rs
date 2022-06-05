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

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

use std::cmp;

impl Solution {
    // 複数参照されつつ、可変でありたい
    pub fn max_depth(root: TreeLink) -> i32 {
        match root {
            Some(node) => {
                let left_depth = 1 + Self::max_depth(node.borrow().left.clone());
                let right_depth = 1 + Self::max_depth(node.borrow().right.clone());

                return cmp::max(left_depth, right_depth)
            },
            _ => {
                return 0
            }
        }
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
    let res = Solution::max_depth(tree);
    println!("{}", res);
}

