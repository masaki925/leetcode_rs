struct Solution {}

impl Solution {
    pub fn a(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut res: i32 = 0;

        while l < r {
            let x_axis = (r - l) as i32;
            let y_axis = nums[l].min(nums[r]);
            let area = x_axis * y_axis;
            res = res.max(area);

            if nums[l] <= nums[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        res
    }
}

fn main() {
  let nums = vec![1,4,3,2,3,4];
  let res = Solution::a(nums);
  println!("{}", res);
}

