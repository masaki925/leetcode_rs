struct Solution {}

impl Solution {
    pub fn a(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in nums {
            res ^= n;
        }
        res
    }
}

fn main() {
  let nums = vec![1,4,3,3,4];
  let res = Solution::a(nums);
  println!("{}", res);
}

