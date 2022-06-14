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

impl Solution {
    pub fn lowest_common_ancestor(root: TreeLink, p: TreeLink, q: TreeLink) -> TreeLink {
        match (root, p, q) {
            (Some(node), Some(pnode), Some(qnode)) => {
                println!("{}, {}, {}", node.borrow().val, pnode.borrow().val, qnode.borrow().val);
                if node.borrow().val > pnode.borrow().val && node.borrow().val > qnode.borrow().val {
                    Self::lowest_common_ancestor(node.borrow().left.clone(), Some(pnode), Some(qnode))
                } else if node.borrow().val < pnode.borrow().val && node.borrow().val < qnode.borrow().val {
                    Self::lowest_common_ancestor(node.borrow().right.clone(), Some(pnode), Some(qnode))
                } else {
                    Some(node)
                }
            },
            _ => {
                None
            }
        }
    }
}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            })))
        })))
    })));

    let p = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None
    })));

    let q = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None
    })));

    let tree = Solution::lowest_common_ancestor(tree, p, q);
    println!("{}", tree.unwrap().borrow().val);
}

