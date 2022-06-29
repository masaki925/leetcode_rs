struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![1];
        let mut prefix = 1;
        let mut suffix = 1;

        for i in 1..nums.len() {
            prefix *= nums[i-1];
            res.push(prefix);
        }

        for i in (0..nums.len()-1).rev() {
            suffix *= nums[i+1];
            res[i] *= suffix;
        }

        res
    }
}

fn main() {
  let nums = vec![1,2,3,4];
  let res = Solution::product_except_self(nums);
  println!("{:?}", res);
}

