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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode{ val: 0, next: None});

        let mut curr = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut v1 = 0;
            let mut v2 = 0;

            if let Some(n1) = l1 {
                v1 = n1.val;
                l1 = n1.next;
            }

            if let Some(n2) = l2 {
                v2 = n2.val;
                l2 = n2.next;
            }

            curr.next = Some(Box::new(ListNode{val: (v1 + v2 + carry) % 10, next: None}));
            carry = (v1 + v2 + carry) / 10;
            curr = curr.next.as_mut().unwrap();
        }

        dummy.next
    }
}

fn main() {
  let l1 = Some(Box::new(ListNode{val: 2, next:
      Some(Box::new(ListNode{val: 4, next:
          Some(Box::new(ListNode{val: 3, next:
              None}))
      }))
  }));

  let l2 = Some(Box::new(ListNode{val: 5, next:
      Some(Box::new(ListNode{val: 6, next:
          Some(Box::new(ListNode{val: 4, next:
              None}))
      }))
  }));

  let res = Solution::add_two_numbers(l1, l2);
  println!("{:?}", res);
}

