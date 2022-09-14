struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut res: i32 = -1;

        while l <= r && r < nums.len() {
            let m = (l + r) / 2;

            if target > nums[m] {
                if nums[m] < nums[l] && target > nums[r] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else if target < nums[m] {
                if nums[m] >= nums[l] && target < nums[l] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            } else {
                res = m as i32;
                break
            }
        }

        res
    }
}

fn main() {
  let nums = vec![4,5,6,7,8,1,2,3];
  let target = 8;
  let res = Solution::search(nums, target);
  println!("{}", res);
}

