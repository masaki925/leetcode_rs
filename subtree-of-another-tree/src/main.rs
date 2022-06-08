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

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(np), Some(nq)) => {
                np.borrow().val == nq.borrow().val
                    && Self::is_same_tree(np.borrow().left.clone(), nq.borrow().left.clone())
                    && Self::is_same_tree(np.borrow().right.clone(), nq.borrow().right.clone())
            },
            _ => false
        }
    }

    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(nr), Some(ns)) => {
                let left  = nr.borrow().left.clone();
                let right = nr.borrow().right.clone();
                if Self::is_same_tree(Some(nr), Some(ns.clone())) {
                    true
                } else {
                    Self::is_subtree(left, Some(ns.clone()))
                        || Self::is_subtree(right, Some(ns.clone()))
                }
            },
            _ => false
        }
    }
}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode { 
                val: 9,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode { 
                        val: 3, left: None, right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode { 
                        val: 4, left: None, right: None
                    })))
                }))),
            })))
        })))
    })));

    let sub = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None
    })));

    let res = Solution::is_subtree(tree, sub);
    println!("{}", res);
}

