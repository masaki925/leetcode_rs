struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut one = 1;
        let mut two = 1;

        if n < 2 {
            return one;
        }

        for _i in 0..n-1 {
            let tmp = one;
            one += two;
            two = tmp;
        }
        one
    }
}

fn main() {
  let n = 4;
  let res = Solution::climb_stairs(n);
  println!("{}", res);
}

