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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(l1), Some(l2)) => {
                if l1.val <= l2.val {
                    Some(Box::new(ListNode{
                        val: l1.val,
                        next: Solution::merge_two_lists(l1.next, Some(l2))
                    }))
                } else {
                    Some(Box::new(ListNode{
                        val: l2.val,
                        next: Solution::merge_two_lists(Some(l1), l2.next)
                    }))
                }
            }
        }
    }

    pub fn print_list(mut list1: Option<Box<ListNode>>) {
        while let Some(n) = list1 {
            println!("{}", n.val);
            list1 = n.next;
        }
    }
}


fn main() {
  let list1 = ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 3, next: None }))}))};
  let list2 = ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 4, next: None }))}))};

  let res = Solution::merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)));
  Solution::print_list(res);
}

