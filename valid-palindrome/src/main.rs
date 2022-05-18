struct Solution {}

impl Solution {
    pub fn a(s: String) -> bool {
        let sv: Vec<char> = s.to_lowercase().chars().collect();
        let mut i = 0;
        let mut j = sv.len() - 1;

        while i < j {
            println!("----");
            println!("{}, {}", i, j);
            println!("{}, {}", sv[i], sv[j]);
            while (i < s.len()) && ! sv[i].is_alphanumeric() {
                i += 1;
                println!("{}, {}", i, j);
            }

            while (j > 0) && ! sv[j].is_alphanumeric() {
                j -= 1;
                println!("{}, {}", i, j);
            }

            if i >= s.len() || j <= 0 {
                break;
            }

            if sv[i] != sv[j] {
                return false;
            }
            i += 1;
            j -= 1;
            println!("{}, {}", i, j);
        }

        true
    }
}

fn main() {
  let s = String::from(",.");
  let res = Solution::a(s);
  println!("{}", res);
}

