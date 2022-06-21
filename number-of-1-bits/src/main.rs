struct Solution {}

impl Solution {
    pub fn hamming_weight (mut n: u32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n % 2;
            n = n >> 1;
        }
        res as i32
    }
}

fn main() {
  let num = 3 as u32;
  let res = Solution::hamming_weight(num);
  println!("{}", res);
}

