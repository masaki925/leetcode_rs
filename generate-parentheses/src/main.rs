struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack: Vec<String> = vec![];
        let mut res: Vec<String> = vec![];

        // open == closed == n
        // open: open < n
        // close: closed < open
        fn backtrack(open_n: i32, closed_n: i32, n: &i32, stack: &mut Vec<String>, res: &mut Vec<String>) {
            if open_n == closed_n && open_n == *n {
                res.push(stack.join(""));
                return
            }

            if open_n < *n {
                stack.push("(".to_string());
                backtrack(open_n + 1, closed_n, &n, stack, res);
                stack.pop();
            }

            if closed_n < open_n {
                stack.push(")".to_string());
                backtrack(open_n, closed_n + 1, &n, stack, res);
                stack.pop();
            }
        }

        backtrack(0, 0, &n, &mut stack, &mut res);
        res
    }
}

fn main() {
  let n = 3;
  let res = Solution::generate_parenthesis(n);
  println!("{:?}", res);
}

