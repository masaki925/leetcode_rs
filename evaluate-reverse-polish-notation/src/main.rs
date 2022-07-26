struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = vec![];

        for t in tokens {
            match &t[..] {
                "+" => {
                    let v1: i32 = stack.pop().unwrap().parse().unwrap();
                    let v2: i32 = stack.pop().unwrap().parse().unwrap();
                    let v3 = v1 + v2;
                    stack.push(v3.to_string());
                },
                "-" => {
                    let v1: i32 = stack.pop().unwrap().parse().unwrap();
                    let v2: i32 = stack.pop().unwrap().parse().unwrap();
                    let v3 = v2 - v1;
                    stack.push(v3.to_string());
                },
                "*" => {
                    let v1: i32 = stack.pop().unwrap().parse().unwrap();
                    let v2: i32 = stack.pop().unwrap().parse().unwrap();
                    let v3 = v2 * v1;
                    stack.push(v3.to_string());
                },
                "/" => {
                    let v1: i32 = stack.pop().unwrap().parse().unwrap();
                    let v2: i32 = stack.pop().unwrap().parse().unwrap();
                    let v3 = v2 / v1;
                    stack.push(v3.to_string());
                },
                _ => {
                    stack.push(t);
                }
            }
        }
        let res: i32 = stack.pop().unwrap().parse().unwrap();
        res
    }
}

fn main() {
  let tokens = vec!["4","13","5","/","+"].iter().map(|v| v.to_string()).collect();
  let res = Solution::eval_rpn(tokens);
  println!("{}", res);
}

