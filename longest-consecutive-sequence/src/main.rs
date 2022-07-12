struct Solution {}

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn a(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut res = 0;

        for n in &nums {
            if set.contains(&(n - 1)) {
                continue;
            }
            let mut len = 1;
            while set.contains(&(n + len)) {
                len += 1;
            }
            res = res.max(len);
        }

        res
    }
}

fn main() {
  let nums = vec![1,4,3,2,3,4];
  let res = Solution::a(nums);
  println!("{}", res);
}

