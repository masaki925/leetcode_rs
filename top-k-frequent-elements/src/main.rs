struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for n in &nums {
            let rec = hm.entry(*n).or_insert(0);
            *rec = *rec + 1;
        }

        let mut tmp: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

        for (k, v) in hm.into_iter() {
            tmp[v as usize].push(k);
        }

        let mut res: Vec<i32> = vec![];
        while res.len() < k as usize {
            let mut t = tmp.pop().unwrap();
            res.append(&mut t);
        }

        res
    }
}

fn main() {
  let nums = vec![1,1,1,2,2,3];
  let k = 2;
  let res = Solution::top_k_frequent(nums, k);
  println!("{:?}", res);
}

