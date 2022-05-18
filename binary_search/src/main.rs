struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len();
        let mut mid;

        while start < end {
            mid = start + ((end - start) / 2);
            println!("{}, {}, {}", start, end, mid);
            if target == nums[mid] {
                return mid as i32;
            } else if target < nums[mid] {
                end = mid;
            } else {
                start = mid + 1;
            }
        }

        -1
    }
}

fn main() {
  let nums = vec![-1,0,3,5,9,12];
  let target = 9;
  let res = Solution::search(nums, target);
  println!("{}", res);
}

