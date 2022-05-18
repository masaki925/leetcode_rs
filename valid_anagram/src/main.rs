struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut sv = Vec::from(s);
        let mut tv = Vec::from(t);
        sv.sort();
        tv.sort();
        sv == tv
    }
}

fn main() {
  let s = String::from("abc");
  let t = String::from("bca");
  let res = Solution::is_anagram(s, t);
  println!("{}", res);
}

