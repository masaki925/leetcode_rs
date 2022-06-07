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
}

fn main() {
    let tree_p = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode { 
                val: 1, left: None, right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode { 
                val: 3, left: None, right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode { 
                val: 6, left: None, right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode { 
                val: 9, left: None, right: None
            })))
        })))
    })));

    let tree_q = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode { 
                val: 1, left: None, right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode { 
                val: 3, left: None, right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode { 
                val: 6, left: None, right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode { 
                val: 9, left: None, right: None
            })))
        })))
    })));

    let res = Solution::is_same_tree(tree_p, tree_q);
    println!("{}", res);
}

