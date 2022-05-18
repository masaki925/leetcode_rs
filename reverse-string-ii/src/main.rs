struct Solution {}

impl Solution {
    pub fn func(s: String, k: i32) -> String {
        let chars:Vec<char> = s.chars().collect::<Vec<_>>();
        let len: i32 = chars.len() as i32;
        let mut new = Vec::new();
        let mut i: i32 = 0;
        let mut reverse = true;

        while i < len {
            let upto = match i+k < len {
                true => i+k,
                false => i + (len - i),
            };
            if reverse {
                for j in (i..upto).rev() {
                    new.push(chars[j as usize]);
                }
            } else {
                for j in i..upto {
                    new.push(chars[j as usize]);
                }
            }
            reverse ^= true;
            i += k;
        }
        new.iter().collect::<String>()
    }
}

fn main() {
  let s = String::from("abcdefg");
  let k = 2;
  let result = Solution::func(s, k);
  println!("{}", result);
}

