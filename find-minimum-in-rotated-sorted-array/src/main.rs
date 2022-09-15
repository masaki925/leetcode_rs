struct Solution {}

impl Solution {
    pub fn a(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut res = 5000;

        while l <= r {
            let m = (l + r) / 2;

            if nums[l] <= nums[m] {
                res = res.min(nums[l]);
                l = m + 1;
            } else {
                res = res.min(nums[m]);
                r = m - 1;
            }
        }

        res
    }
}

fn main() {
  let nums = vec![11,13,15,17];
  let res = Solution::a(nums);
  println!("{}", res);
}

