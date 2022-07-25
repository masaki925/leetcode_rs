struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut l = 0;
        let mut set: HashSet<char> = HashSet::new();

        for (r, x) in s.chars().enumerate() {
            while l < r && set.contains(&x) {
                let hoge = s.chars().nth(l).unwrap();
                set.remove(&hoge);
                l += 1;
            }
            set.insert(x);
            res = res.max((r as i32) - (l as i32) + 1);
        }

        res
    }
}

fn main() {
  let s = "hogho".to_string();
  let res = Solution::length_of_longest_substring(s);
  println!("{}", res);
}

