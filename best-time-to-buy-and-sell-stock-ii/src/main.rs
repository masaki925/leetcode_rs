struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
      let mut total = 0;

      for i in 0..prices.len() - 1 {
        if prices[i] < prices[i+1] {
          total += prices[i+1] - prices[i];
        }
      }

      total
    }
}

fn main() {
  let prices = vec![1,4,3,2,3,4];
  let profit = Solution::max_profit(prices);
  println!("{}", profit);
}

