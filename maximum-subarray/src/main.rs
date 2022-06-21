struct Solution {}

impl Solution {
    pub fn a(nums: Vec<i32>) -> i32 {
        let mut cur_sum = 0;
        let mut max_sum = nums[0];

        for n in nums {
            if cur_sum < 0 {
                cur_sum = 0;
            }
            cur_sum += n;
            max_sum = i32::max(max_sum, cur_sum);
        }
        max_sum
    }
}

fn main() {
  // let nums = vec![1,-4,3,-2,3,4];
  let nums = vec![-1];
  let res = Solution::a(nums);
  println!("{}", res);
}

