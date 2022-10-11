
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Link
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

use std::collections::VecDeque;

type Link = Option<Box<ListNode>>;

struct Solution {}

impl Solution {
    pub fn reorder_list(mut head: &mut Link) {
        let mut queue: VecDeque<Link> = VecDeque::new();

        let mut first_node = head.as_mut().unwrap().next.take();

        while let Some(mut node) = first_node {
            first_node = node.next.take();
            queue.push_back(Some(node));
        }

        while !queue.is_empty() {
            if let Some(last) = queue.pop_back() {
                head.as_mut().unwrap().next = last;
                head = &mut head.as_mut().unwrap().next;
            }

            if let Some(next) = queue.pop_front() {
                head.as_mut().unwrap().next = next;
                head = &mut head.as_mut().unwrap().next;
            }
        }
    }
}

fn main() {
    let mut head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }))
    }));

    Solution::reorder_list(&mut head);
    println!("{:?}", head);
}

