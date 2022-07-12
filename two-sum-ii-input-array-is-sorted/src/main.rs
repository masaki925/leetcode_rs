struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;

        while l < r {
            if (numbers[l] + numbers[r]) == target {
                return vec![(l + 1) as i32,(r + 1) as i32];
            } else if (numbers[l] + numbers[r]) < target {
                l += 1;
            } else {
                r -= 1;
            }
        }

        vec![0,0]
    }
}

fn main() {
  let nums = vec![2,7,11,15];
  let target = 9;
  let res = Solution::two_sum(nums, target);
  println!("{:?}", res);
}

