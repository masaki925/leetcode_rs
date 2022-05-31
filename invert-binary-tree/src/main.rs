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

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution {}

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    // 複数参照されつつ、可変でありたい
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let l2 = node.borrow().left.clone();
            let r2 = node.borrow().right.clone();

            node.borrow_mut().left = Self::invert_tree(r2);
            node.borrow_mut().right = Self::invert_tree(l2);

            return Some(node);
        }

        None
    }

    pub fn print_tree(root: TreeLink) {
        let mut queue: VecDeque<TreeLink> = VecDeque::new();

        queue.push_front(root);

        while let Some(curr) = queue.pop_back() {
            match curr {
                Some(node) => {
                    println!("{}", node.borrow().val);
                    queue.push_front(node.borrow().left.clone());
                    queue.push_front(node.borrow().right.clone());
                },
                _ => {}
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
    let res = Solution::invert_tree(tree);
    Solution::print_tree(res);
}

