struct Solution {}

impl Solution {
    pub fn a(prices: Vec<i32>) -> i32 {
        let mut min_price  = prices[0];
        let mut max_profit = 0;

        for price in prices {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }
}

fn main() {
  //let nums = vec![7,1,5,3,6,4];
  let nums = vec![1,2];
  let res = Solution::a(nums);
  println!("{}", res);
}

