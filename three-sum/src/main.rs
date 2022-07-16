struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        nums.sort();

        for mut i in 0..nums.len() {
            if i > 0 && nums[i-1] == nums[i] {
                continue;
            }
            let t1 = 0 - nums[i];

            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let sum = nums[j] + nums[k];
                if sum < t1 {
                    j += 1;
                } else if sum > t1 {
                    k -= 1;
                } else {
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    while j < k && nums[j - 1] == nums[j] {
                        j += 1;
                    }
                }
            }
        }

        res
    }
}

fn main() {
  //let nums = vec![-1,0,1,2,-1,-4];
  let nums = vec![0,0,0,0];
  let res = Solution::three_sum(nums);
  println!("{:?}", res);
}

