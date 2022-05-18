fn main() {
    let mut nums = vec![1,1,2,3,3,3];
    println!("result: {}", remove_dup(&mut nums));
}

fn remove_dup(nums: &mut Vec<i32>) -> i32 {
  let (mut i, mut p, n) = (0, 0, nums.len());
  while i < n {
    nums[p] = nums[i];
    while i < n && nums[i] == nums[p] {
      i += 1;
    }
    p += 1;
  }
  return p as i32;
}
