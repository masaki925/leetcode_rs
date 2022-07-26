struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut t1 = vec![0; 26];
        let mut t2 = vec![0; 26];

        if s1.len() > s2.len() {
            return false;
        }

        for c in s1.chars() {
            let offset = c as u16 - 'a' as u16;
            t1[offset as usize] += 1;
        }

        for i in 0..s1.len() {
            let offset = s2.chars().nth(i).unwrap() as u16 - 'a' as u16;
            t2[offset as usize] += 1;
        }

        if t1 == t2 {
            return true;
        }

        let mut l = 0;
        for r in s1.len()..s2.len() {
            let o_r = s2.chars().nth(r).unwrap() as u16 - 'a' as u16;
            t2[o_r as usize] += 1;

            let o_l = s2.chars().nth(l).unwrap() as u16 - 'a' as u16;
            t2[o_l as usize] -= 1;
            l += 1;

            if t1 == t2 {
                return true;
            }
        }

        false
    }
}

fn main() {
  let s1 = "ab".to_string();
  let s2 = "eidboaoo".to_string();
  let res = Solution::check_inclusion(s1, s2);
  println!("{}", res);
}

