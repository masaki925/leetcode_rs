use std::collections::HashSet;

struct Solution {}

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();

    if nums.len() == 1 {
        return nums[0];
    }

    for n in nums {
        if !set.remove(&n) {
            set.insert(n);
        }
    }
    return set.into_iter().collect::<Vec<_>>()[0];
  }
}

fn main() {
  let nums = vec![1,2,3,2,1];
  let result = Solution::contains_duplicate(nums);
  println!("{}", result);
}

