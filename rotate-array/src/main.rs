struct Solution {}

impl Solution {
  pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
    let n = nums.len();
    k = k % (n as i32);

    let mut new = Vec::from(&nums[n - (k as usize)..]);
    new.append(&mut Vec::from(&nums[..n - (k as usize)]));
    *nums = new;
  }
}

fn main() {
  let (mut nums, k) = (vec![1,2,3,4,5,6,7], 3);
  Solution::rotate(&mut nums, k);
  println!("{:?}", nums);
}

