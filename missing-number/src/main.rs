struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let sum_all = n * (n+1) / 2;

        let sum_nums: i32 = nums.iter().sum();

        (sum_all as i32) - sum_nums
    }
}

fn main() {
  let nums = vec![0,1,3];
  let res = Solution::missing_number(nums);
  println!("{}", res);
}

