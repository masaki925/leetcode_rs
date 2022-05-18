struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                return vec![map[&nums[i]], i as i32];
            }
            map.entry(target - nums[i]).or_insert(i as i32);
        }
        vec![0,0]
    }
}

fn main() {
  let nums = vec![3,4,1,2,3,4];
  let target = 5;
  let res = Solution::two_sum(nums, target);
  println!("{:?}", res);
}

