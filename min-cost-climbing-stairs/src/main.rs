struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        for (idx, i) in (0..cost.len()).rev().enumerate() {
            if idx < 2 {
                continue
            }
            cost[i] += i32::min(cost[i+1], cost[i+2]);
        }
        i32::min(cost[0], cost[1])
    }
}

fn main() {
  let nums = vec![1,4,3,2,3,4];
  let res = Solution::min_cost_climbing_stairs(nums);
  println!("{}", res);
}

