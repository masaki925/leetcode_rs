struct Solution {}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max_p = piles.iter().max().unwrap();

        let mut l = 1;
        let mut r = *max_p;
        let mut k = *max_p;

        while l <= r {
            let try_k = ((l as f64 + r as f64) / 2.0).round() as i32;

            let mut try_h = 0;
            for b_num in &piles {
                try_h += (*b_num as f64 / try_k as f64).ceil() as i32;
            }

            if try_h <= h {
                k = k.min(try_k);
                r = try_k - 1;
            } else {
                l = try_k + 1;
            }
        }

        k
    }
}

fn main() {
  let piles = vec![312884470];
  let h = 312884469;
  let res = Solution::min_eating_speed(piles, h);
  println!("{}", res);
}

