struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut offset = 1;
        let mut dp: Vec<i32> = vec![0];

        for i in 1..n+1 {
            if offset * 2 == i {
                offset = i;
            }
            dp.push(1 + dp[(i - offset) as usize]);
        }
        dp
    }
}

fn main() {
  let n = 5;
  let res = Solution::count_bits(n);
  println!("{:?}", res);
}

