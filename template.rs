struct Solution {}

impl Solution {
    pub fn a(nums: Vec<i32>) -> i32 {
        nums[0]
    }
}

fn main() {
  let nums = vec![1,4,3,2,3,4];
  let res = Solution::a(nums);
  println!("{}", res);
}

