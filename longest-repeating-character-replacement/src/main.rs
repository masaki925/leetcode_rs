struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut l = 0;
        let mut maxf = 0;
        let mut map: HashMap<char, i32> = HashMap::new();

        for (r, c) in s.chars().enumerate() {
            println!("########## {}, {}", r, c);
            *map.entry(c).or_insert(0) += 1;
            maxf = maxf.max(map[&c]);

            println!("{}, {}, {}, {}", l, r, maxf, k);
            while l < r && ((r as i32) - (l as i32) + 1 - maxf) > k {
                *map.entry(s.chars().nth(l).unwrap()).or_insert(0) -= 1;
                l += 1;
                println!("l: {}", l);
            }
            res = res.max((r as i32) - (l as i32) + 1);
            println!("res: {}", res);
        }
        res
    }
}

fn main() {
  let s = "ABAA".to_string();
  let k = 0;
  let res = Solution::character_replacement(s, k);
  println!("{}", res);
}

