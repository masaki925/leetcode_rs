struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack: Vec<(usize, &i32)> = vec![];

        for (i, t) in temperatures.iter().enumerate() {
            while let Some(last) = stack.last() {
                if *(*last).1 < *t {
                    res[(*last).0] = (i - (*last).0) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((i, t));
        }

        res
    }
}

fn main() {
  let nums = vec![1,4,3,2,3,4];
  let res = Solution::daily_temperatures(nums);
  println!("{:?}", res);
}

