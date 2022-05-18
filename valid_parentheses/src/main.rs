struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => {
                    stack.push(c);
                },
                ')' => {
                    match stack.pop() {
                        Some('(') => {
                            continue;
                        },
                        _ => {
                            return false;
                        }
                    };
                },
                '}' => {
                    match stack.pop() {
                        Some('{') => {
                            continue;
                        },
                        _ => {
                            return false;
                        }
                    };
                },
                ']' => {
                    match stack.pop() {
                        Some('[') => {
                            continue;
                        },
                        _ => {
                            return false;
                        }
                    };
               },
                _ => {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

fn main() {
  let s = String::from("()");
  let res = Solution::is_valid(s);
  println!("{}", res);
}

