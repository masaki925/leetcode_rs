use std::collections::HashSet;

struct Solution {}

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashSet::new();
    for n in nums {
      if !map.insert(n) {
        return true;
      }
    }
    return false;
  }
}

fn main() {
  let nums = vec![1,2,3,1,1,1];
  let result = Solution::contains_duplicate(nums);
  println!("{}", result);
}

