// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
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

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr: Option<Box<ListNode>> = head;

        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    pub fn print_list(head: Option<Box<ListNode>>) {
        match head {
            Some(head) => {
                println!("{}", head.val);
                match head.next {
                    Some(node) => {
                        Solution::print_list(Some(node));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

fn main() {
  let nums = vec![1,2,3,4,5];
  let mut prev_ln: Option<Box<ListNode>> = None;
  for i in (0..nums.len()).rev() {
      let mut ln = ListNode::new(nums[i]);
      match prev_ln {
          Some(prev) => {
              if i != nums.len() {
                  ln.next = Some(prev);
              }
          },
          _ => {}
      }
      prev_ln = Some(Box::new(ln));
  }
  let res = Solution::reverse_list(prev_ln);
  Solution::print_list(res);
}

