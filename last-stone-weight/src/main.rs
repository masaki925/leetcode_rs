struct Solution {}

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);

        while heap.len() > 1 {
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();
            if x != y {
                let diff = i32::abs(x - y);
                heap.push(diff);
            }
        }

        heap.push(0);
        heap.pop().unwrap()
    }
}

fn main() {
  let nums = vec![];
  let res = Solution::last_stone_weight(nums);
  println!("{}", res);
}

